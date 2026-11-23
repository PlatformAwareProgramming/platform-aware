use std::ffi::c_void;
use std::mem::{size_of, transmute};
use std::ptr;
use std::sync::OnceLock;

use crate::plugin::DynamicPlugin;
use crate::{check, initial_grid};

const HOST_TO_DEVICE: i32 = 1;
const DEVICE_TO_HOST: i32 = 2;
const ROCM_PLUGIN_ENV: &str = "HEAT_ROCM_LIBRARY";
const ROCM_PLUGIN_LIB: &str = "libheat_rocm.so";

struct RocmApi {
    _plugin: DynamicPlugin,
    hip_malloc: unsafe extern "C" fn(*mut *mut c_void, usize) -> i32,
    hip_free: unsafe extern "C" fn(*mut c_void) -> i32,
    hip_memcpy: unsafe extern "C" fn(*mut c_void, *const c_void, usize, i32) -> i32,
    hip_memset: unsafe extern "C" fn(*mut c_void, i32, usize) -> i32,
    hip_device_synchronize: unsafe extern "C" fn() -> i32,
    heat_rocm_launch: unsafe extern "C" fn(*const f32, *mut f32, i32, f32, u32, u32) -> i32,
}

impl RocmApi {
    fn load() -> Result<Self, String> {
        let plugin = DynamicPlugin::load(ROCM_PLUGIN_ENV, ROCM_PLUGIN_LIB)?;
        unsafe {
            Ok(Self {
                hip_malloc: transmute(plugin.symbol_raw("hipMalloc")?),
                hip_free: transmute(plugin.symbol_raw("hipFree")?),
                hip_memcpy: transmute(plugin.symbol_raw("hipMemcpy")?),
                hip_memset: transmute(plugin.symbol_raw("hipMemset")?),
                hip_device_synchronize: transmute(plugin.symbol_raw("hipDeviceSynchronize")?),
                heat_rocm_launch: transmute(plugin.symbol_raw("heat_rocm_launch")?),
                _plugin: plugin,
            })
        }
    }
}

static ROCM_API: OnceLock<RocmApi> = OnceLock::new();

pub fn solve_heat_rocm_impl(n: usize, steps: usize, alpha: f32) -> Vec<f32> {
    assert!(n >= 3 && (0.0..=0.5).contains(&alpha));
    let api = ROCM_API.get_or_init(|| {
        RocmApi::load().unwrap_or_else(|e| panic!("ROCm kernel selected, but {e}"))
    });

    let mut host = initial_grid(n);
    let bytes = n * size_of::<f32>();
    let (mut u, mut next) = (ptr::null_mut(), ptr::null_mut());

    unsafe {
        check((api.hip_malloc)(&mut u, bytes), "hipMalloc(u)");
        check((api.hip_malloc)(&mut next, bytes), "hipMalloc(next)");
        check(
            (api.hip_memcpy)(u, host.as_ptr().cast(), bytes, HOST_TO_DEVICE),
            "hipMemcpy(u)",
        );
        check((api.hip_memset)(next, 0, bytes), "hipMemset(next)");

        let threads = 256_u32;
        let blocks = ((n as u32 - 2) + threads - 1) / threads;
        for _ in 0..steps {
            check(
                (api.heat_rocm_launch)(u.cast(), next.cast(), n as i32, alpha, blocks, threads),
                "heat_rocm_launch",
            );
            std::mem::swap(&mut u, &mut next);
        }
        check((api.hip_device_synchronize)(), "hipDeviceSynchronize");
        check(
            (api.hip_memcpy)(host.as_mut_ptr().cast(), u, bytes, DEVICE_TO_HOST),
            "hipMemcpy(result)",
        );
        check((api.hip_free)(u), "hipFree(u)");
        check((api.hip_free)(next), "hipFree(next)");
    }
    host
}
