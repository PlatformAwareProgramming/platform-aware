#include <cuda_runtime.h>

extern "C" __global__ void heat_cuda_kernel(
    const float *u, float *next, int n, float alpha)
{
    int i = blockIdx.x * blockDim.x + threadIdx.x + 1;
    if (i < n - 1)
        next[i] = u[i] + alpha * (u[i - 1] - 2.0f * u[i] + u[i + 1]);
}

// Rust can launch a CUDA function through cudaLaunchKernel but cannot name a
// __global__ symbol. This tiny bridge only exposes that symbol's address.
extern "C" void *heat_cuda_kernel_address() {
    return reinterpret_cast<void *>(heat_cuda_kernel);
}

