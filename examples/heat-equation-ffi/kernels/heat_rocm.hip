#include <hip/hip_runtime.h>

extern "C" __global__ void heat_rocm_kernel(
    const float *u, float *next, int n, float alpha)
{
    int i = blockIdx.x * blockDim.x + threadIdx.x + 1;
    if (i < n - 1)
        next[i] = u[i] + alpha * (u[i - 1] - 2.0f * u[i] + u[i + 1]);
}

// As on CUDA, this bridge exposes only the vendor kernel symbol to Rust.
extern "C" void *heat_rocm_kernel_address() {
    return reinterpret_cast<void *>(heat_rocm_kernel);
}

