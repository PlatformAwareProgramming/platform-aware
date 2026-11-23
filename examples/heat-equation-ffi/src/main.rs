mod cuda_impl;
mod plugin;
mod rocm_impl;

use cuda_impl::solve_heat_cuda_impl;
use platform_aware::platformaware;
use rocm_impl::solve_heat_rocm_impl;

pub(crate) fn initial_grid(n: usize) -> Vec<f32> {
    let mut u = vec![0.0; n];
    u[n / 2] = 1.0; // a unit heat impulse at the centre
    u
}

pub(crate) fn check(code: i32, operation: &str) {
    assert_eq!(code, 0, "{operation} failed with GPU error {code}");
}

#[platformaware]
mod heat_solver_equation {
    use platform_aware::AtLeast;
    use platform_aware_amd::ROCM;
    use platform_aware_nvidia::CUDA;

    // Fallback: explicit finite differences executed sequentially on the CPU.
    #[kernelversion]
    pub fn solve_heat(n: usize, steps: usize, alpha: f32) -> Vec<f32> {
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
        super::solve_heat_cuda_impl(n, steps, alpha)
    }

    #[kernelversion(acc_count=AtLeast{val: 1}, acc_backend=ROCM)]
    pub fn solve_heat(n: usize, steps: usize, alpha: f32) -> Vec<f32> {
        super::solve_heat_rocm_impl(n, steps, alpha)
    }
}

fn main() {
    let answer = heat_solver_equation::solve_heat(1024, 100, 0.4);
    println!("centre after 100 steps: {}", answer[512]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_fallback_conserves_dirichlet_boundaries_and_diffuses_heat() {
        let answer = heat_solver_equation::solve_heat_variant_0(1024, 100, 0.4);
        assert_eq!(answer.len(), 1024);
        assert_eq!(answer[0], 0.0);
        assert_eq!(answer[1023], 0.0);
        assert!(answer[512] > 0.0 && answer[512] < 1.0);
        assert!((answer[511] - answer[513]).abs() < 1e-6);
        let total_heat: f32 = answer.iter().sum();
        assert!((total_heat - 1.0).abs() < 1e-4);
    }
}
