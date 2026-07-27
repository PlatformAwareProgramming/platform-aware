#include <hip/hip_runtime.h>

extern "C" __global__ void heat_rocm_kernel(
    const float *u, float *next, int n, float alpha)
{
    int i = blockIdx.x * blockDim.x + threadIdx.x + 1;
    if (i < n - 1)
        next[i] = u[i] + alpha * (u[i - 1] - 2.0f * u[i] + u[i + 1]);
}

// Keep HIP's C++ dim3 and kernel-argument ABI on the HIP side of the FFI
// boundary. Rust only passes C-compatible pointers and scalar values.
extern "C" int heat_rocm_launch(
    const float *u, float *next, int n, float alpha,
    unsigned int blocks, unsigned int threads)
{
    hipLaunchKernelGGL(
        heat_rocm_kernel, dim3(blocks), dim3(threads), 0, 0,
        u, next, n, alpha);
    return static_cast<int>(hipGetLastError());
}
