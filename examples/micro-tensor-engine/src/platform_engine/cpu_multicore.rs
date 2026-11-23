// Platform module: multicore specializations (scalar, AVX2+FMA, AVX-512)

impl TensorOps for Tensor {
    #[kernelversion(cpu_core_count = AtLeast { val: 2 })]
    fn active_backend(&self) -> &'static str {
        "multicore_scalar"
    }

    #[kernelversion(cpu_simd = AVX2, cpu_core_count = AtLeast { val: 2 })]
    fn active_backend(&self) -> &'static str {
        "multicore_avx2"
    }

    #[kernelversion(cpu_simd = AVX512, cpu_core_count = AtLeast { val: 2 })]
    fn active_backend(&self) -> &'static str {
        "multicore_avx512"
    }

    #[kernelversion(cpu_core_count = AtLeast { val: 2 })]
    fn dot(&self, rhs: &Tensor) -> f32 {
        multicore::scalar_mt::dot(self, rhs)
    }

    #[kernelversion(cpu_simd = AVX2, cpu_core_count = AtLeast { val: 2 })]
    fn dot(&self, rhs: &Tensor) -> f32 {
        multicore::avx2_mt::dot(self, rhs)
    }

    #[kernelversion(cpu_simd = AVX512, cpu_core_count = AtLeast { val: 2 })]
    fn dot(&self, rhs: &Tensor) -> f32 {
        multicore::avx512_mt::dot(self, rhs)
    }

    #[kernelversion(cpu_core_count = AtLeast { val: 2 })]
    fn axpy(&mut self, alpha: f32, x: &Tensor) {
        multicore::scalar_mt::axpy(alpha, x, self);
    }

    #[kernelversion(cpu_simd = AVX2, cpu_core_count = AtLeast { val: 2 })]
    fn axpy(&mut self, alpha: f32, x: &Tensor) {
        multicore::avx2_mt::axpy(alpha, x, self);
    }

    #[kernelversion(cpu_simd = AVX512, cpu_core_count = AtLeast { val: 2 })]
    fn axpy(&mut self, alpha: f32, x: &Tensor) {
        multicore::avx512_mt::axpy(alpha, x, self);
    }

    #[kernelversion(cpu_core_count = AtLeast { val: 2 })]
    fn relu(&self) -> Tensor {
        multicore::scalar_mt::relu(self)
    }

    #[kernelversion(cpu_simd = AVX2, cpu_core_count = AtLeast { val: 2 })]
    fn relu(&self) -> Tensor {
        multicore::avx2_mt::relu(self)
    }

    #[kernelversion(cpu_simd = AVX512, cpu_core_count = AtLeast { val: 2 })]
    fn relu(&self) -> Tensor {
        multicore::avx512_mt::relu(self)
    }

    #[kernelversion(cpu_core_count = AtLeast { val: 2 })]
    fn matvec(&self, x: &Tensor) -> Tensor {
        multicore::scalar_mt::matvec(self, x)
    }

    #[kernelversion(cpu_simd = AVX2, cpu_core_count = AtLeast { val: 2 })]
    fn matvec(&self, x: &Tensor) -> Tensor {
        multicore::avx2_mt::matvec(self, x)
    }

    #[kernelversion(cpu_simd = AVX512, cpu_core_count = AtLeast { val: 2 })]
    fn matvec(&self, x: &Tensor) -> Tensor {
        multicore::avx512_mt::matvec(self, x)
    }

    #[kernelversion(cpu_core_count = AtLeast { val: 2 })]
    fn matmul(&self, rhs: &Tensor) -> Tensor {
        multicore::scalar_mt::matmul(self, rhs)
    }

    #[kernelversion(cpu_simd = AVX2, cpu_core_count = AtLeast { val: 2 })]
    fn matmul(&self, rhs: &Tensor) -> Tensor {
        multicore::avx2_mt::matmul(self, rhs)
    }

    #[kernelversion(cpu_simd = AVX512, cpu_core_count = AtLeast { val: 2 })]
    fn matmul(&self, rhs: &Tensor) -> Tensor {
        multicore::avx512_mt::matmul(self, rhs)
    }
}
