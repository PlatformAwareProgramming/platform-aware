# Heat equation using CPU fallback, CUDA, and ROCm shared-library plugins

This example advances the explicit finite-difference discretization

`u[i] <- u[i] + alpha * (u[i-1] - 2*u[i] + u[i+1])`

with fixed zero Dirichlet boundary values (`u[0] = u[n-1] = 0`). Stability requires `0 <= alpha <= 0.5`.

The public kernel function is `heat_solver_equation::solve_heat`. The `#[platformaware]` macro defines three versions:

- `#[kernelversion]` — portable CPU **fallback** version (executes on any platform).
- `#[kernelversion(acc_count=AtLeast{val: 1}, acc_backend=CUDA)]` — delegates to `cuda_impl::solve_heat_cuda_impl`, calling the CUDA Runtime API and the `heat_cuda_launch` bridge via dynamic loading.
- `#[kernelversion(acc_count=AtLeast{val: 1}, acc_backend=ROCM)]` — delegates to `rocm_impl::solve_heat_rocm_impl`, calling the HIP Runtime API and the `heat_rocm_launch` bridge via dynamic loading.

Small kernel-launch bridges (`heat_cuda_launch`, `heat_rocm_launch`) are compiled as vendor source so that C++ `dim3` and kernel-argument ABI details never cross Rust's FFI boundary. There is no common GPU abstraction or generic GPU implementation.

## Avoiding Mandatory Accelerator Dependencies

`build.rs` compiles `kernels/heat_cuda.cu` and `kernels/heat_rocm.hip` into two separate backend plugins beside the executable (`libheat_cuda.so` and `libheat_rocm.so`) whenever `nvcc` and/or `hipcc` are available on `PATH`. The executable itself is not linked against either vendor runtime (`libcudart` or `libamdhip64`). After platform-aware dispatch selects a kernel version, only the corresponding plugin is dynamically loaded and cached.

Only the selected plugin and its vendor runtime need to be installed on the target machine. For example, an AMD installation only needs the executable and `libheat_rocm.so`, without `libheat_cuda.so` or the CUDA runtime; a CPU-only machine runs the fallback without either plugin.

## Building

```sh
cargo build --release
```

If `CUDA_PATH`, `ROCM_PATH`, `HIP_GCC_INSTALL_DIR`, or `CXX` need custom locations, supply them via environment variables. If neither `nvcc` nor `hipcc` is installed, the build still succeeds and runs the CPU fallback version.

## Running

Place an appropriate `Platform.toml` in the working directory (or set `PLATFORM_DESCRIPTION`):

- CPU fallback: omit `Platform.toml`, set `acc_count = 0`, or omit GPU backends.
- NVIDIA GPU: set `acc_count >= 1` and include `"CUDA"` (or a CUDA subtype such as `"CUDA120"`) in `acc_backend`.
- AMD GPU: set `acc_count >= 1` and include `"ROCM"` in `acc_backend`.

By default, plugins are searched beside the executable. `HEAT_CUDA_LIBRARY` and `HEAT_ROCM_LIBRARY` override their respective full paths, and `HEAT_PLUGIN_DIR` overrides the plugin search directory.

```sh
./target/release/heat-equation-ffi
```
