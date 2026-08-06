use platform_aware::platformaware;

#[platformaware]
mod heat_solver_equation {
    use platform_aware_amd::ROCM;
    use platform_aware_nvidia::CUDA;
    use platform_aware::{AtLeast};

    // Fallback: explicit finite differences executed sequentially on the CPU.
    #[kernelversion]
    pub fn solve_heat(n: usize, steps: usize, alpha: f32) -> Vec<f32> {
        println!("fall-back version");
        assert!(n >= 3 && (0.0..=0.5).contains(&alpha));
        let mut u = super::initial_grid(n);
        let mut next = vec![0.0; n];

        for _ in 0..steps {
            // Fixed Dirichlet boundary conditions: u[0] = u[n - 1] = 0.
            for i in 1..n - 1 {
                next[i] = u[i] + alpha * (u[i - 1] - 2.0 * u[i] + u[i + 1]);
            }
            std::mem::swap(&mut u, &mut next);
        }

        u
    }

    #[kernelversion(acc_count=AtLeast{val: 1}, acc_backend=CUDA)]
    pub fn solve_heat(n: usize, steps: usize, alpha: f32) -> Vec<f32> {
        println!("CUDA version");
        super::solve_heat_cuda_impl(n, steps, alpha)
    }

    #[kernelversion(acc_count=AtLeast{val: 1}, acc_backend=ROCM)]
    pub fn solve_heat(n: usize, steps: usize, alpha: f32) -> Vec<f32> {
        println!("ROCM version");
        super::solve_heat_rocm_impl(n, steps, alpha)
    }
}

fn main() {
    let answer = heat_solver_equation::solve_heat(1024, 100, 0.4);
    println!("centre after 100 steps: {}", answer[512]);
}

use std::{
    env,
    ffi::{c_char, c_int, c_void, CStr, CString},
    mem::size_of,
    path::{Path, PathBuf},
    ptr,
};

const HOST_TO_DEVICE: i32 = 1;
const DEVICE_TO_HOST: i32 = 2;

fn initial_grid(n: usize) -> Vec<f32> {
    let mut u = vec![0.0; n];
    u[n / 2] = 1.0; // a unit heat impulse at the centre
    u
}

fn check(code: i32, operation: &str) {
    assert_eq!(code, 0, "{operation} failed with GPU error {code}");
}

const RTLD_NOW: c_int = 2;
const RTLD_LOCAL: c_int = 0;

#[link(name = "dl")]
extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
    fn dlerror() -> *const c_char;
}

struct DynamicLibrary(*mut c_void);

impl DynamicLibrary {
    fn open(environment: &str, filename: &str) -> Self {
        let path = env::var_os(environment)
            .map(PathBuf::from)
            .unwrap_or_else(|| sibling_library(filename));
        let path = CString::new(path.as_os_str().as_encoded_bytes())
            .unwrap_or_else(|_| panic!("invalid plugin path for {filename}"));
        let handle = unsafe { dlopen(path.as_ptr(), RTLD_NOW | RTLD_LOCAL) };
        if handle.is_null() {
            panic!("cannot load {filename}: {}", dynamic_loader_error());
        }
        Self(handle)
    }

    unsafe fn symbol<T: Copy>(&self, name: &str) -> T {
        let name = CString::new(name).expect("symbol contains a NUL byte");
        dlerror();
        let address = dlsym(self.0, name.as_ptr());
        let error = dlerror();
        if !error.is_null() {
            panic!(
                "cannot load symbol {}: {}",
                name.to_string_lossy(),
                CStr::from_ptr(error).to_string_lossy()
            );
        }
        std::mem::transmute_copy(&address)
    }
}

impl Drop for DynamicLibrary {
    fn drop(&mut self) {
        unsafe {
            dlclose(self.0);
        }
    }
}

fn sibling_library(filename: &str) -> PathBuf {
    env::current_exe()
        .ok()
        .and_then(|executable| executable.parent().map(Path::to_path_buf))
        .unwrap_or_default()
        .join(filename)
}

fn dynamic_loader_error() -> String {
    unsafe {
        let error = dlerror();
        if error.is_null() {
            "unknown dynamic loader error".into()
        } else {
            CStr::from_ptr(error).to_string_lossy().into_owned()
        }
    }
}

type Malloc = unsafe extern "C" fn(*mut *mut c_void, usize) -> i32;
type Free = unsafe extern "C" fn(*mut c_void) -> i32;
type Memcpy = unsafe extern "C" fn(*mut c_void, *const c_void, usize, i32) -> i32;
type Memset = unsafe extern "C" fn(*mut c_void, i32, usize) -> i32;
type Synchronize = unsafe extern "C" fn() -> i32;
type Launch = unsafe extern "C" fn(*const f32, *mut f32, i32, f32, u32, u32) -> i32;

fn solve_heat_cuda_impl(n: usize, steps: usize, alpha: f32) -> Vec<f32> {
    assert!(n >= 3 && (0.0..=0.5).contains(&alpha));
    let mut host = initial_grid(n);
    let bytes = n * size_of::<f32>();
    let (mut u, mut next) = (ptr::null_mut(), ptr::null_mut());

    unsafe {
        let library = DynamicLibrary::open("HEAT_CUDA_LIBRARY", "libheat_cuda.so");
        let malloc: Malloc = library.symbol("cudaMalloc");
        let free: Free = library.symbol("cudaFree");
        let memcpy: Memcpy = library.symbol("cudaMemcpy");
        let memset: Memset = library.symbol("cudaMemset");
        let synchronize: Synchronize = library.symbol("cudaDeviceSynchronize");
        let launch: Launch = library.symbol("heat_cuda_launch");
        check(malloc(&mut u, bytes), "cudaMalloc(u)");
        check(malloc(&mut next, bytes), "cudaMalloc(next)");
        check(
            memcpy(u, host.as_ptr().cast(), bytes, HOST_TO_DEVICE),
            "cudaMemcpy(u)",
        );
        check(memset(next, 0, bytes), "cudaMemset(next)");

        let threads = 256_u32;
        let blocks = ((n as u32 - 2) + threads - 1) / threads;
        for _ in 0..steps {
            // Dirichlet boundary values u[0] = u[n-1] = 0 remain untouched.
            check(
                launch(u.cast(), next.cast(), n as i32, alpha, blocks, threads),
                "heat_cuda_launch",
            );
            std::mem::swap(&mut u, &mut next);
        }
        check(synchronize(), "cudaDeviceSynchronize");
        check(
            memcpy(host.as_mut_ptr().cast(), u, bytes, DEVICE_TO_HOST),
            "cudaMemcpy(result)",
        );
        check(free(u), "cudaFree(u)");
        check(free(next), "cudaFree(next)");
    }
    host
}

fn solve_heat_rocm_impl(n: usize, steps: usize, alpha: f32) -> Vec<f32> {
    assert!(n >= 3 && (0.0..=0.5).contains(&alpha));
    let mut host = initial_grid(n);
    let bytes = n * size_of::<f32>();
    let (mut u, mut next) = (ptr::null_mut(), ptr::null_mut());

    unsafe {
        let library = DynamicLibrary::open("HEAT_ROCM_LIBRARY", "libheat_rocm.so");
        let malloc: Malloc = library.symbol("hipMalloc");
        let free: Free = library.symbol("hipFree");
        let memcpy: Memcpy = library.symbol("hipMemcpy");
        let memset: Memset = library.symbol("hipMemset");
        let synchronize: Synchronize = library.symbol("hipDeviceSynchronize");
        let launch: Launch = library.symbol("heat_rocm_launch");
        check(malloc(&mut u, bytes), "hipMalloc(u)");
        check(malloc(&mut next, bytes), "hipMalloc(next)");
        check(
            memcpy(u, host.as_ptr().cast(), bytes, HOST_TO_DEVICE),
            "hipMemcpy(u)",
        );
        check(memset(next, 0, bytes), "hipMemset(next)");

        let threads = 256_u32;
        let blocks = ((n as u32 - 2) + threads - 1) / threads;
        for _ in 0..steps {
            check(
                launch(u.cast(), next.cast(), n as i32, alpha, blocks, threads),
                "heat_rocm_launch",
            );
            std::mem::swap(&mut u, &mut next);
        }
        check(synchronize(), "hipDeviceSynchronize");
        check(
            memcpy(host.as_mut_ptr().cast(), u, bytes, DEVICE_TO_HOST),
            "hipMemcpy(result)",
        );
        check(free(u), "hipFree(u)");
        check(free(next), "hipFree(next)");
    }
    host
}
