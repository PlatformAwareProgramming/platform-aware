//! Structured `platform-aware` tensor engine using k-methods and external platform modules
//! (Section 4.7 of "Structured platform-aware programming with Rust").

use platform_aware::platformaware;

use crate::tensor::Tensor;

/// Uniform platform-agnostic tensor API exposed to callers.
pub trait TensorOps {
    fn active_backend(&self) -> &'static str;
    fn dot(&self, rhs: &Tensor) -> f32;
    fn axpy(&mut self, alpha: f32, x: &Tensor);
    fn relu(&self) -> Tensor;
    fn matvec(&self, x: &Tensor) -> Tensor;
    fn matmul(&self, rhs: &Tensor) -> Tensor;
}

#[platformaware]
pub mod ops {
    use platform_aware::{AVX2, AVX512, AtLeast, SSE_4_2};

    use super::{Tensor, TensorOps};
    use crate::kernels::{avx2, avx512, multicore, scalar, sse42};

    // Lightweight k-function used to measure first-call resolution latency
    // and amortized per-call dispatch overhead (Section 5.3).
    #[kernelversion]
    #[inline(never)]
    pub fn dispatch_probe(x: u64) -> u64 {
        x.wrapping_add(1)
    }

    #[kernelversion(cpu_simd = SSE_4_2)]
    #[inline(never)]
    pub fn dispatch_probe(x: u64) -> u64 {
        x.wrapping_add(1)
    }

    #[kernelversion(cpu_simd = AVX2)]
    #[inline(never)]
    pub fn dispatch_probe(x: u64) -> u64 {
        x.wrapping_add(1)
    }

    #[kernelversion(cpu_core_count = AtLeast { val: 2 })]
    #[inline(never)]
    pub fn dispatch_probe(x: u64) -> u64 {
        x.wrapping_add(1)
    }

    #[kernelversion(cpu_simd = AVX2, cpu_core_count = AtLeast { val: 2 })]
    #[inline(never)]
    pub fn dispatch_probe(x: u64) -> u64 {
        x.wrapping_add(1)
    }

    #[kernelversion(cpu_simd = AVX512, cpu_core_count = AtLeast { val: 2 })]
    #[inline(never)]
    pub fn dispatch_probe(x: u64) -> u64 {
        x.wrapping_add(1)
    }

    // Fallback (portable scalar) implementation in the enclosing kernel module.
    impl TensorOps for Tensor {
        #[kernelversion]
        fn active_backend(&self) -> &'static str {
            "scalar"
        }

        #[kernelversion]
        fn dot(&self, rhs: &Tensor) -> f32 {
            scalar::dot(self, rhs)
        }

        #[kernelversion]
        fn axpy(&mut self, alpha: f32, x: &Tensor) {
            scalar::axpy(alpha, x, self);
        }

        #[kernelversion]
        fn relu(&self) -> Tensor {
            scalar::relu(self)
        }

        #[kernelversion]
        fn matvec(&self, x: &Tensor) -> Tensor {
            scalar::matvec(self, x)
        }

        #[kernelversion]
        fn matmul(&self, rhs: &Tensor) -> Tensor {
            scalar::matmul(self, rhs)
        }
    }

    // External platform modules (Listing 11 / Section 4.7)
    platform!(cpu_simd);
    platform!(cpu_multicore);
}
