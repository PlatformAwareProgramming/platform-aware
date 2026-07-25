use std::{ffi::c_void, mem::size_of, ptr};

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

fn solve_heat_cuda_impl(n: usize, steps: usize, alpha: f32) -> Vec<f32> {
    assert!(n >= 3 && (0.0..=0.5).contains(&alpha));
    let mut host = initial_grid(n);
    let bytes = n * size_of::<f32>();
    let (mut u, mut next) = (ptr::null_mut(), ptr::null_mut());

    unsafe {
        check(cuda::cudaMalloc(&mut u, bytes), "cudaMalloc(u)");
        check(cuda::cudaMalloc(&mut next, bytes), "cudaMalloc(next)");
        check(
            cuda::cudaMemcpy(u, host.as_ptr().cast(), bytes, HOST_TO_DEVICE),
            "cudaMemcpy(u)",
        );
        check(cuda::cudaMemset(next, 0, bytes), "cudaMemset(next)");

        let threads = 256_u32;
        let blocks = ((n as u32 - 2) + threads - 1) / threads;
        for _ in 0..steps {
            // Dirichlet boundary values u[0] = u[n-1] = 0 remain untouched.
            let mut n_arg = n as i32;
            let mut alpha_arg = alpha;
            let mut args = [
                (&mut u as *mut *mut c_void).cast(),
                (&mut next as *mut *mut c_void).cast(),
                (&mut n_arg as *mut i32).cast(),
                (&mut alpha_arg as *mut f32).cast(),
            ];
            check(
                cuda::cudaLaunchKernel(
                    cuda::heat_cuda_kernel_address(),
                    Dim3::x(blocks),
                    Dim3::x(threads),
                    0,
                    ptr::null_mut(),
                    args.as_mut_ptr(),
                ),
                "cudaLaunchKernel",
            );
            std::mem::swap(&mut u, &mut next);
        }
        check(cuda::cudaDeviceSynchronize(), "cudaDeviceSynchronize");
        check(
            cuda::cudaMemcpy(host.as_mut_ptr().cast(), u, bytes, DEVICE_TO_HOST),
            "cudaMemcpy(result)",
        );
        check(cuda::cudaFree(u), "cudaFree(u)");
        check(cuda::cudaFree(next), "cudaFree(next)");
    }
    host
}

fn solve_heat_rocm_impl(n: usize, steps: usize, alpha: f32) -> Vec<f32> {
    assert!(n >= 3 && (0.0..=0.5).contains(&alpha));
    let mut host = initial_grid(n);
    let bytes = n * size_of::<f32>();
    let (mut u, mut next) = (ptr::null_mut(), ptr::null_mut());

    unsafe {
        check(rocm::hipMalloc(&mut u, bytes), "hipMalloc(u)");
        check(rocm::hipMalloc(&mut next, bytes), "hipMalloc(next)");
        check(
            rocm::hipMemcpy(u, host.as_ptr().cast(), bytes, HOST_TO_DEVICE),
            "hipMemcpy(u)",
        );
        check(rocm::hipMemset(next, 0, bytes), "hipMemset(next)");

        let threads = 256_u32;
        let blocks = ((n as u32 - 2) + threads - 1) / threads;
        for _ in 0..steps {
            let mut n_arg = n as i32;
            let mut alpha_arg = alpha;
            let mut args = [
                (&mut u as *mut *mut c_void).cast(),
                (&mut next as *mut *mut c_void).cast(),
                (&mut n_arg as *mut i32).cast(),
                (&mut alpha_arg as *mut f32).cast(),
            ];
            check(
                rocm::hipLaunchKernel(
                    rocm::heat_rocm_kernel_address(),
                    Dim3::x(blocks),
                    Dim3::x(threads),
                    args.as_mut_ptr(),
                    0,
                    ptr::null_mut(),
                ),
                "hipLaunchKernel",
            );
            std::mem::swap(&mut u, &mut next);
        }
        check(rocm::hipDeviceSynchronize(), "hipDeviceSynchronize");
        check(
            rocm::hipMemcpy(host.as_mut_ptr().cast(), u, bytes, DEVICE_TO_HOST),
            "hipMemcpy(result)",
        );
        check(rocm::hipFree(u), "hipFree(u)");
        check(rocm::hipFree(next), "hipFree(next)");
    }
    host
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Dim3 {
    x: u32,
    y: u32,
    z: u32,
}

impl Dim3 {
    fn x(x: u32) -> Self {
        Self { x, y: 1, z: 1 }
    }
}

mod cuda {
    use std::ffi::c_void;
    extern "C" {
        pub fn heat_cuda_kernel_address() -> *const c_void;
        pub fn cudaMalloc(p: *mut *mut c_void, bytes: usize) -> i32;
        pub fn cudaFree(p: *mut c_void) -> i32;
        pub fn cudaMemcpy(dst: *mut c_void, src: *const c_void, bytes: usize, kind: i32) -> i32;
        pub fn cudaMemset(dst: *mut c_void, value: i32, bytes: usize) -> i32;
        pub fn cudaLaunchKernel(
            f: *const c_void,
            grid: super::Dim3,
            block: super::Dim3,
            shared: usize,
            stream: *mut c_void,
            args: *mut *mut c_void,
        ) -> i32;
        pub fn cudaDeviceSynchronize() -> i32;
    }
}

mod rocm {
    use std::ffi::c_void;
    extern "C" {
        pub fn heat_rocm_kernel_address() -> *const c_void;
        pub fn hipMalloc(p: *mut *mut c_void, bytes: usize) -> i32;
        pub fn hipFree(p: *mut c_void) -> i32;
        pub fn hipMemcpy(dst: *mut c_void, src: *const c_void, bytes: usize, kind: i32) -> i32;
        pub fn hipMemset(dst: *mut c_void, value: i32, bytes: usize) -> i32;
        pub fn hipLaunchKernel(
            f: *const c_void,
            grid: super::Dim3,
            block: super::Dim3,
            args: *mut *mut c_void,
            shared: usize,
            stream: *mut c_void,
        ) -> i32;
        pub fn hipDeviceSynchronize() -> i32;
    }
}

use platform_aware::platformaware;

#[platformaware]
mod heat_solver_equation {
    use platform_aware_amd::ROCM;
    use platform_aware_nvidia::CUDA;

    #[kernelversion(acc_backend=CUDA)]
    pub fn solve_heat(n: usize, steps: usize, alpha: f32) -> Vec<f32> {
        super::solve_heat_cuda_impl(n, steps, alpha)
    }

    #[kernelversion(acc_backend=ROCM)]
    pub fn solve_heat(n: usize, steps: usize, alpha: f32) -> Vec<f32> {
        super::solve_heat_rocm_impl(n, steps, alpha)
    }
}

fn main() {
    let answer = heat_solver_equation::solve_heat(1024, 100, 0.4);
    println!("centre after 100 steps: {}", answer[512]);
}
