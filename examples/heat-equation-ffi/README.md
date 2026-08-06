# Heat equation using CUDA and ROCm through FFI

This example advances the explicit finite-difference discretization

`u[i] <- u[i] + alpha * (u[i-1] - 2*u[i] + u[i+1])`

with fixed zero boundary values. Stability requires `0 <= alpha <= 0.5`.
The public kernel function is `heat_solver_equation::solve_heat`. The
`#[platformaware]` macro gives it two versions: `acc_backend=CUDA` calls the
CUDA Runtime API, while `acc_backend=ROCM` calls the HIP Runtime API through
raw FFI. Small kernel-launch bridges are compiled as vendor source so that
the C++ `dim3` and kernel-argument ABI does not cross Rust's FFI boundary.
There is no common GPU abstraction or generic GPU implementation.

Build once on a machine or build container that has both the CUDA and ROCm
toolkits installed (`nvcc` and `hipcc` must be on `PATH`):

```sh
cargo build --release
```

If the SDK library directories are not configured system-wide, supply both to
the linker, for example:

```sh
LIBRARY_PATH=/usr/local/cuda/lib64:/opt/rocm/lib cargo build --release
```

The platform description must identify the backend available on the machine,
for example `acc_backend = ["CUDA"]` or `acc_backend = ["ROCM"]` in the
accelerator section of `Platform.toml`. The build creates the executable and
two backend plugins beside it: `libheat_cuda.so` and `libheat_rocm.so`. The
executable selects the matching version and dynamically opens only that
version's plugin, so it has no startup dependency on either vendor runtime.
No Cargo feature or recompilation is involved in that selection.

Only the selected plugin and its vendor runtime need to be installed. For
example, an AMD installation can contain the executable and
`libheat_rocm.so`, without `libheat_cuda.so` or the CUDA runtime. By default,
plugins are found beside the executable. `HEAT_CUDA_LIBRARY` and
`HEAT_ROCM_LIBRARY` can override their respective full paths.

Run that same executable on either platform after placing the appropriate
`Platform.toml` in the working directory:

```sh
./target/release/heat-equation-ffi
```
