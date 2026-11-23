use std::ffi::c_void;
use std::mem::{size_of, transmute};
use std::ptr;
use std::sync::OnceLock;

use crate::plugin::DynamicPlugin;
use crate::{check, initial_grid};

const HOST_TO_DEVICE: i32 = 1;
const DEVICE_TO_HOST: i32 = 2;
const CUDA_PLUGIN_ENV: &str = "HEAT_CUDA_LIBRARY";
const CUDA_PLUGIN_LIB: &str = "libheat_cuda.so";

struct CudaApi {
    _plugin: DynamicPlugin,
    cuda_malloc: unsafe extern "C" fn(*mut *mut c_void, usize) -> i32,
    cuda_free: unsafe extern "C" fn(*mut c_void) -> i32,
    cuda_memcpy: unsafe extern "C" fn(*mut c_void, *const c_void, usize, i32) -> i32,
    cuda_memset: unsafe extern "C" fn(*mut c_void, i32, usize) -> i32,
    cuda_device_synchronize: unsafe extern "C" fn() -> i32,
    heat_cuda_launch: unsafe extern "C" fn(*const f32, *mut f32, i32, f32, u32, u32) -> i32,
}

impl CudaApi {
    fn load() -> Result<Self, String> {
        let plugin = DynamicPlugin::load(CUDA_PLUGIN_ENV, CUDA_PLUGIN_LIB)?;
        unsafe {
            Ok(Self {
                cuda_malloc: transmute(plugin.symbol_raw("cudaMalloc")?),
                cuda_free: transmute(plugin.symbol_raw("cudaFree")?),
                cuda_memcpy: transmute(plugin.symbol_raw("cudaMemcpy")?),
                cuda_memset: transmute(plugin.symbol_raw("cudaMemset")?),
                cuda_device_synchronize: transmute(plugin.symbol_raw("cudaDeviceSynchronize")?),
                heat_cuda_launch: transmute(plugin.symbol_raw("heat_cuda_launch")?),
                _plugin: plugin,
            })
        }
    }
}

static CUDA_API: OnceLock<CudaApi> = OnceLock::new();

pub fn solve_heat_cuda_impl(n: usize, steps: usize, alpha: f32) -> Vec<f32> {
    assert!(n >= 3 && (0.0..=0.5).contains(&alpha));
    let api = CUDA_API.get_or_init(|| {
        CudaApi::load().unwrap_or_else(|e| panic!("CUDA kernel selected, but {e}"))
    });

    let mut host = initial_grid(n);
    let bytes = n * size_of::<f32>();
    let (mut u, mut next) = (ptr::null_mut(), ptr::null_mut());

    unsafe {
        check((api.cuda_malloc)(&mut u, bytes), "cudaMalloc(u)");
        check((api.cuda_malloc)(&mut next, bytes), "cudaMalloc(next)");
        check(
            (api.cuda_memcpy)(u, host.as_ptr().cast(), bytes, HOST_TO_DEVICE),
            "cudaMemcpy(u)",
        );
        check((api.cuda_memset)(next, 0, bytes), "cudaMemset(next)");

        let threads = 256_u32;
        let blocks = ((n as u32 - 2) + threads - 1) / threads;
        for _ in 0..steps {
            // Dirichlet boundary values u[0] = u[n-1] = 0 remain untouched.
            check(
                (api.heat_cuda_launch)(u.cast(), next.cast(), n as i32, alpha, blocks, threads),
                "heat_cuda_launch",
            );
            std::mem::swap(&mut u, &mut next);
        }
        check((api.cuda_device_synchronize)(), "cudaDeviceSynchronize");
        check(
            (api.cuda_memcpy)(host.as_mut_ptr().cast(), u, bytes, DEVICE_TO_HOST),
            "cudaMemcpy(result)",
        );
        check((api.cuda_free)(u), "cudaFree(u)");
        check((api.cuda_free)(next), "cudaFree(next)");
    }
    host
}
