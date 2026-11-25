// Platform module: NVIDIA Turing GPU specialization via CUDA Driver API + inline PTX
// (requires `acc_backend = CUDA` and `acc_architecture = Turing` in `Platform.toml`).

impl TensorOps for Tensor {
    #[kernelversion(acc_backend = CUDA, acc_architecture = Turing)]
    fn active_backend(&self) -> &'static str {
        "gpu_cuda"
    }

    #[kernelversion(acc_backend = CUDA, acc_architecture = Turing)]
    fn dot(&self, rhs: &Tensor) -> f32 {
        cuda_backend::dot(self, rhs)
    }

    #[kernelversion(acc_backend = CUDA, acc_architecture = Turing)]
    fn axpy(&mut self, alpha: f32, x: &Tensor) {
        cuda_backend::axpy(alpha, x, self);
    }

    #[kernelversion(acc_backend = CUDA, acc_architecture = Turing)]
    fn relu(&self) -> Tensor {
        cuda_backend::relu(self)
    }

    #[kernelversion(acc_backend = CUDA, acc_architecture = Turing)]
    fn matvec(&self, x: &Tensor) -> Tensor {
        cuda_backend::matvec(self, x)
    }

    #[kernelversion(acc_backend = CUDA, acc_architecture = Turing)]
    fn matmul(&self, rhs: &Tensor) -> Tensor {
        cuda_backend::matmul(self, rhs)
    }
}
