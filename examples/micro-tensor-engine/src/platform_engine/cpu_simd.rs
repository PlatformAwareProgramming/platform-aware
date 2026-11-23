// Platform module: single-core SIMD specializations (SSE4.2, AVX2+FMA, AVX-512)

impl TensorOps for Tensor {
    #[kernelversion(cpu_simd = SSE_4_2)]
    fn active_backend(&self) -> &'static str {
        "sse42"
    }

    #[kernelversion(cpu_simd = AVX2)]
    fn active_backend(&self) -> &'static str {
        "avx2"
    }

    #[kernelversion(cpu_simd = AVX512)]
    fn active_backend(&self) -> &'static str {
        "avx512"
    }

    #[kernelversion(cpu_simd = SSE_4_2)]
    fn dot(&self, rhs: &Tensor) -> f32 {
        sse42::dot(self, rhs)
    }

    #[kernelversion(cpu_simd = AVX2)]
    fn dot(&self, rhs: &Tensor) -> f32 {
        avx2::dot(self, rhs)
    }

    #[kernelversion(cpu_simd = AVX512)]
    fn dot(&self, rhs: &Tensor) -> f32 {
        avx512::dot(self, rhs)
    }

    #[kernelversion(cpu_simd = SSE_4_2)]
    fn axpy(&mut self, alpha: f32, x: &Tensor) {
        sse42::axpy(alpha, x, self);
    }

    #[kernelversion(cpu_simd = AVX2)]
    fn axpy(&mut self, alpha: f32, x: &Tensor) {
        avx2::axpy(alpha, x, self);
    }

    #[kernelversion(cpu_simd = AVX512)]
    fn axpy(&mut self, alpha: f32, x: &Tensor) {
        avx512::axpy(alpha, x, self);
    }

    #[kernelversion(cpu_simd = SSE_4_2)]
    fn relu(&self) -> Tensor {
        sse42::relu(self)
    }

    #[kernelversion(cpu_simd = AVX2)]
    fn relu(&self) -> Tensor {
        avx2::relu(self)
    }

    #[kernelversion(cpu_simd = AVX512)]
    fn relu(&self) -> Tensor {
        avx512::relu(self)
    }

    #[kernelversion(cpu_simd = SSE_4_2)]
    fn matvec(&self, x: &Tensor) -> Tensor {
        sse42::matvec(self, x)
    }

    #[kernelversion(cpu_simd = AVX2)]
    fn matvec(&self, x: &Tensor) -> Tensor {
        avx2::matvec(self, x)
    }

    #[kernelversion(cpu_simd = AVX512)]
    fn matvec(&self, x: &Tensor) -> Tensor {
        avx512::matvec(self, x)
    }

    #[kernelversion(cpu_simd = SSE_4_2)]
    fn matmul(&self, rhs: &Tensor) -> Tensor {
        sse42::matmul(self, rhs)
    }

    #[kernelversion(cpu_simd = AVX2)]
    fn matmul(&self, rhs: &Tensor) -> Tensor {
        avx2::matmul(self, rhs)
    }

    #[kernelversion(cpu_simd = AVX512)]
    fn matmul(&self, rhs: &Tensor) -> Tensor {
        avx512::matmul(self, rhs)
    }
}
