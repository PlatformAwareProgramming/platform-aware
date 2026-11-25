//! Real NVIDIA GPU backend via the CUDA Driver API (`libcuda.so.1`) and inline PTX (`sm_75`, Turing).
//!
//! Dynamically loads `libcuda.so.1` via glibc `dlopen`/`dlsym` at runtime so:
//!   1. Zero external Cargo crates and zero `nvcc` installation are required.
//!   2. The compiled `bench_pa` binary remains portable even on CPU-only machines.
//!   3. On hosts with an NVIDIA GPU (e.g., GeForce RTX 2070, Turing `sm_75`), the NVIDIA
//!      driver's built-in PTX JIT compiles and executes all five `TensorOps` kernels natively.

use std::ffi::{CStr, c_char, c_int, c_uint, c_void};
use std::ptr;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

use crate::tensor::Tensor;

type CuResult = c_int;
type CuDevice = c_int;
type CuContext = *mut c_void;
type CuModule = *mut c_void;
type CuFunction = *mut c_void;
type CuStream = *mut c_void;
type CuDevicePtr = u64;

const CUDA_SUCCESS: CuResult = 0;
const RTLD_LAZY: c_int = 0x0001;
const RTLD_LOCAL: c_int = 0x0000;

extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

type FnCuInit = unsafe extern "C" fn(c_uint) -> CuResult;
type FnCuDeviceGetCount = unsafe extern "C" fn(*mut c_int) -> CuResult;
type FnCuDeviceGet = unsafe extern "C" fn(*mut CuDevice, c_int) -> CuResult;
type FnCuDeviceGetName = unsafe extern "C" fn(*mut c_char, c_int, CuDevice) -> CuResult;
type FnCuCtxCreate = unsafe extern "C" fn(*mut CuContext, c_uint, CuDevice) -> CuResult;
type FnCuCtxSetCurrent = unsafe extern "C" fn(CuContext) -> CuResult;
type FnCuCtxSynchronize = unsafe extern "C" fn() -> CuResult;
type FnCuModuleLoadData = unsafe extern "C" fn(*mut CuModule, *const c_void) -> CuResult;
type FnCuModuleGetFunction =
    unsafe extern "C" fn(*mut CuFunction, CuModule, *const c_char) -> CuResult;
type FnCuMemAlloc = unsafe extern "C" fn(*mut CuDevicePtr, usize) -> CuResult;
type FnCuMemFree = unsafe extern "C" fn(CuDevicePtr) -> CuResult;
type FnCuMemcpyHtoD = unsafe extern "C" fn(CuDevicePtr, *const c_void, usize) -> CuResult;
type FnCuMemcpyDtoH = unsafe extern "C" fn(*mut c_void, CuDevicePtr, usize) -> CuResult;
type FnCuLaunchKernel = unsafe extern "C" fn(
    CuFunction,
    c_uint,
    c_uint,
    c_uint,
    c_uint,
    c_uint,
    c_uint,
    c_uint,
    CuStream,
    *mut *mut c_void,
    *mut *mut c_void,
) -> CuResult;

/// Inline PTX 7.0 (`sm_75`, Turing) kernels for all five dense tensor operations:
/// `k_axpy`, `k_relu`, `k_dot_partial`, `k_matvec`, and 16x16 shared-memory tiled `k_matmul_tiled`.
const PTX_SOURCE: &str = r#"
.version 7.0
.target sm_75
.address_size 64

.visible .entry k_axpy(
    .param .u64 p_x,
    .param .u64 p_y,
    .param .f32 p_alpha,
    .param .u32 p_n
) {
    .reg .pred %p<2>;
    .reg .u32 %r<5>;
    .reg .u64 %rd<7>;
    .reg .f32 %f<4>;

    ld.param.u64 %rd1, [p_x];
    ld.param.u64 %rd2, [p_y];
    ld.param.f32 %f1, [p_alpha];
    ld.param.u32 %r1, [p_n];

    mov.u32 %r2, %ctaid.x;
    mov.u32 %r3, %ntid.x;
    mov.u32 %r4, %tid.x;
    mad.lo.u32 %r2, %r2, %r3, %r4;
    setp.ge.u32 %p1, %r2, %r1;
    @%p1 bra DONE_AXPY;

    mul.wide.u32 %rd3, %r2, 4;
    add.u64 %rd4, %rd1, %rd3;
    add.u64 %rd5, %rd2, %rd3;
    ld.global.f32 %f2, [%rd4];
    ld.global.f32 %f3, [%rd5];
    fma.rn.f32 %f3, %f1, %f2, %f3;
    st.global.f32 [%rd5], %f3;

DONE_AXPY:
    ret;
}

.visible .entry k_relu(
    .param .u64 p_x,
    .param .u64 p_o,
    .param .u32 p_n
) {
    .reg .pred %p<2>;
    .reg .u32 %r<5>;
    .reg .u64 %rd<7>;
    .reg .f32 %f<3>;

    ld.param.u64 %rd1, [p_x];
    ld.param.u64 %rd2, [p_o];
    ld.param.u32 %r1, [p_n];

    mov.u32 %r2, %ctaid.x;
    mov.u32 %r3, %ntid.x;
    mov.u32 %r4, %tid.x;
    mad.lo.u32 %r2, %r2, %r3, %r4;
    setp.ge.u32 %p1, %r2, %r1;
    @%p1 bra DONE_RELU;

    mul.wide.u32 %rd3, %r2, 4;
    add.u64 %rd4, %rd1, %rd3;
    add.u64 %rd5, %rd2, %rd3;
    ld.global.f32 %f1, [%rd4];
    mov.f32 %f2, 0f00000000;
    max.f32 %f1, %f1, %f2;
    st.global.f32 [%rd5], %f1;

DONE_RELU:
    ret;
}

// Block-strided dot product reduction (256 threads/block). Writes one partial sum per block.
.visible .entry k_dot_partial(
    .param .u64 p_a,
    .param .u64 p_b,
    .param .u64 p_out,
    .param .u32 p_n
) {
    .shared .align 4 .f32 sh_dot[256];
    .reg .pred %p<4>;
    .reg .u32 %r<10>;
    .reg .u64 %rd<10>;
    .reg .f32 %f<5>;

    ld.param.u64 %rd1, [p_a];
    ld.param.u64 %rd2, [p_b];
    ld.param.u64 %rd3, [p_out];
    ld.param.u32 %r1, [p_n];

    mov.u32 %r2, %tid.x;
    mov.u32 %r3, %ctaid.x;
    mov.u32 %r4, %ntid.x;
    mov.u32 %r5, %nctaid.x;

    mad.lo.u32 %r6, %r3, %r4, %r2;   // idx
    mul.lo.u32 %r7, %r4, %r5;        // stride = gridDim.x * blockDim.x
    mov.f32 %f1, 0f00000000;         // acc

DOT_LOOP:
    setp.ge.u32 %p1, %r6, %r1;
    @%p1 bra DOT_REDUCE;
    mul.wide.u32 %rd4, %r6, 4;
    add.u64 %rd5, %rd1, %rd4;
    add.u64 %rd6, %rd2, %rd4;
    ld.global.f32 %f2, [%rd5];
    ld.global.f32 %f3, [%rd6];
    fma.rn.f32 %f1, %f2, %f3, %f1;
    add.u32 %r6, %r6, %r7;
    bra DOT_LOOP;

DOT_REDUCE:
    mov.u64 %rd7, sh_dot;
    mul.wide.u32 %rd8, %r2, 4;
    add.u64 %rd9, %rd7, %rd8;
    st.shared.f32 [%rd9], %f1;
    bar.sync 0;

    mov.u32 %r8, 128;
DOT_TREE:
    setp.eq.u32 %p2, %r8, 0;
    @%p2 bra DOT_WRITE;
    setp.ge.u32 %p3, %r2, %r8;
    @%p3 bra DOT_SKIP;
    add.u32 %r9, %r2, %r8;
    mul.wide.u32 %rd4, %r9, 4;
    add.u64 %rd5, %rd7, %rd4;
    ld.shared.f32 %f2, [%rd9];
    ld.shared.f32 %f3, [%rd5];
    add.f32 %f2, %f2, %f3;
    st.shared.f32 [%rd9], %f2;
DOT_SKIP:
    bar.sync 0;
    shr.u32 %r8, %r8, 1;
    bra DOT_TREE;

DOT_WRITE:
    setp.ne.u32 %p1, %r2, 0;
    @%p1 bra DOT_DONE;
    ld.shared.f32 %f4, [%rd7];
    mul.wide.u32 %rd4, %r3, 4;
    add.u64 %rd5, %rd3, %rd4;
    st.global.f32 [%rd5], %f4;

DOT_DONE:
    ret;
}

// One 256-thread block per matrix row for coalesced matvec.
.visible .entry k_matvec(
    .param .u64 p_a,
    .param .u64 p_x,
    .param .u64 p_y,
    .param .u32 p_m,
    .param .u32 p_k
) {
    .shared .align 4 .f32 sh_mv[256];
    .reg .pred %p<4>;
    .reg .u32 %r<10>;
    .reg .u64 %rd<12>;
    .reg .f32 %f<5>;

    ld.param.u64 %rd1, [p_a];
    ld.param.u64 %rd2, [p_x];
    ld.param.u64 %rd3, [p_y];
    ld.param.u32 %r1, [p_m];
    ld.param.u32 %r2, [p_k];

    mov.u32 %r3, %ctaid.x;   // row
    setp.ge.u32 %p1, %r3, %r1;
    @%p1 bra MV_DONE;

    mov.u32 %r4, %tid.x;     // col stride start
    mul.wide.u32 %rd10, %r3, %r2;
    shl.b64 %rd10, %rd10, 2;
    add.u64 %rd11, %rd1, %rd10; // &A[row * k]

    mov.u32 %r5, %r4;        // j = tid.x
    mov.f32 %f1, 0f00000000;

MV_LOOP:
    setp.ge.u32 %p1, %r5, %r2;
    @%p1 bra MV_REDUCE;
    mul.wide.u32 %rd4, %r5, 4;
    add.u64 %rd5, %rd11, %rd4;
    add.u64 %rd6, %rd2, %rd4;
    ld.global.f32 %f2, [%rd5];
    ld.global.f32 %f3, [%rd6];
    fma.rn.f32 %f1, %f2, %f3, %f1;
    add.u32 %r5, %r5, 256;
    bra MV_LOOP;

MV_REDUCE:
    mov.u64 %rd7, sh_mv;
    mul.wide.u32 %rd8, %r4, 4;
    add.u64 %rd9, %rd7, %rd8;
    st.shared.f32 [%rd9], %f1;
    bar.sync 0;

    mov.u32 %r6, 128;
MV_TREE:
    setp.eq.u32 %p2, %r6, 0;
    @%p2 bra MV_WRITE;
    setp.ge.u32 %p3, %r4, %r6;
    @%p3 bra MV_SKIP;
    add.u32 %r7, %r4, %r6;
    mul.wide.u32 %rd4, %r7, 4;
    add.u64 %rd5, %rd7, %rd4;
    ld.shared.f32 %f2, [%rd9];
    ld.shared.f32 %f3, [%rd5];
    add.f32 %f2, %f2, %f3;
    st.shared.f32 [%rd9], %f2;
MV_SKIP:
    bar.sync 0;
    shr.u32 %r6, %r6, 1;
    bra MV_TREE;

MV_WRITE:
    setp.ne.u32 %p1, %r4, 0;
    @%p1 bra MV_DONE;
    ld.shared.f32 %f4, [%rd7];
    mul.wide.u32 %rd4, %r3, 4;
    add.u64 %rd5, %rd3, %rd4;
    st.global.f32 [%rd5], %f4;

MV_DONE:
    ret;
}

// 16x16 shared-memory tiled SGEMM with exact bounds handling for arbitrary M, K, N.
.visible .entry k_matmul_tiled(
    .param .u64 p_a,
    .param .u64 p_b,
    .param .u64 p_c,
    .param .u32 p_m,
    .param .u32 p_k,
    .param .u32 p_n
) {
    .shared .align 4 .f32 tile_a[256];
    .shared .align 4 .f32 tile_b[256];

    .reg .pred %p<6>;
    .reg .u32 %r<22>;
    .reg .u64 %rd<16>;
    .reg .f32 %f<6>;

    ld.param.u64 %rd1, [p_a];
    ld.param.u64 %rd2, [p_b];
    ld.param.u64 %rd3, [p_c];
    ld.param.u32 %r1, [p_m];
    ld.param.u32 %r2, [p_k];
    ld.param.u32 %r3, [p_n];

    mov.u32 %r4, %tid.x;    // tx (0..15)
    mov.u32 %r5, %tid.y;    // ty (0..15)
    mov.u32 %r6, %ctaid.x;  // bx
    mov.u32 %r7, %ctaid.y;  // by

    shl.b32 %r8, %r7, 4;
    add.u32 %r8, %r8, %r5;  // row = by*16 + ty
    shl.b32 %r9, %r6, 4;
    add.u32 %r9, %r9, %r4;  // col = bx*16 + tx

    // sh_idx = ty*16 + tx
    shl.b32 %r10, %r5, 4;
    add.u32 %r10, %r10, %r4;
    mul.wide.u32 %rd4, %r10, 4;
    mov.u64 %rd5, tile_a;
    mov.u64 %rd6, tile_b;
    add.u64 %rd7, %rd5, %rd4; // &tile_a[ty][tx]
    add.u64 %rd8, %rd6, %rd4; // &tile_b[ty][tx]

    // num_tiles = (K + 15) >> 4
    add.u32 %r11, %r2, 15;
    shr.u32 %r11, %r11, 4;

    mov.f32 %f1, 0f00000000; // acc
    mov.u32 %r12, 0;         // t = 0

TILE_LOOP:
    setp.ge.u32 %p1, %r12, %r11;
    @%p1 bra TILE_DONE;

    // Load A[row, t*16 + tx]
    shl.b32 %r13, %r12, 4;
    add.u32 %r14, %r13, %r4; // a_col = t*16 + tx
    setp.lt.u32 %p2, %r8, %r1;
    setp.lt.u32 %p3, %r14, %r2;
    and.pred %p2, %p2, %p3;
    mov.f32 %f2, 0f00000000;
    @!%p2 bra SKIP_LA;
    mul.wide.u32 %rd9, %r8, %r2;
    cvt.u64.u32 %rd10, %r14;
    add.u64 %rd9, %rd9, %rd10;
    shl.b64 %rd9, %rd9, 2;
    add.u64 %rd9, %rd1, %rd9;
    ld.global.f32 %f2, [%rd9];
SKIP_LA:
    st.shared.f32 [%rd7], %f2;

    // Load B[t*16 + ty, col]
    add.u32 %r15, %r13, %r5; // b_row = t*16 + ty
    setp.lt.u32 %p4, %r15, %r2;
    setp.lt.u32 %p5, %r9, %r3;
    and.pred %p4, %p4, %p5;
    mov.f32 %f3, 0f00000000;
    @!%p4 bra SKIP_LB;
    mul.wide.u32 %rd11, %r15, %r3;
    cvt.u64.u32 %rd12, %r9;
    add.u64 %rd11, %rd11, %rd12;
    shl.b64 %rd11, %rd11, 2;
    add.u64 %rd11, %rd2, %rd11;
    ld.global.f32 %f3, [%rd11];
SKIP_LB:
    st.shared.f32 [%rd8], %f3;

    bar.sync 0;

    // Inner dot over 16 elements of tile
    shl.b32 %r16, %r5, 4; // ty*16
    mov.u32 %r17, 0;      // kk = 0
INNER_K:
    setp.ge.u32 %p2, %r17, 16;
    @%p2 bra INNER_DONE;
    add.u32 %r18, %r16, %r17; // ty*16 + kk
    shl.b32 %r19, %r17, 4;
    add.u32 %r19, %r19, %r4;  // kk*16 + tx
    mul.wide.u32 %rd13, %r18, 4;
    mul.wide.u32 %rd14, %r19, 4;
    add.u64 %rd13, %rd5, %rd13;
    add.u64 %rd14, %rd6, %rd14;
    ld.shared.f32 %f4, [%rd13];
    ld.shared.f32 %f5, [%rd14];
    fma.rn.f32 %f1, %f4, %f5, %f1;
    add.u32 %r17, %r17, 1;
    bra INNER_K;

INNER_DONE:
    bar.sync 0;
    add.u32 %r12, %r12, 1;
    bra TILE_LOOP;

TILE_DONE:
    setp.lt.u32 %p2, %r8, %r1;
    setp.lt.u32 %p3, %r9, %r3;
    and.pred %p2, %p2, %p3;
    @!%p2 bra MATMUL_RET;
    mul.wide.u32 %rd15, %r8, %r3;
    cvt.u64.u32 %rd10, %r9;
    add.u64 %rd15, %rd15, %rd10;
    shl.b64 %rd15, %rd15, 2;
    add.u64 %rd15, %rd3, %rd15;
    st.global.f32 [%rd15], %f1;

MATMUL_RET:
    ret;
}
"#;

struct CudaApi {
    _lib: *mut c_void,
    cu_ctx_set_current: FnCuCtxSetCurrent,
    cu_ctx_synchronize: FnCuCtxSynchronize,
    cu_mem_alloc: FnCuMemAlloc,
    cu_mem_free: FnCuMemFree,
    cu_memcpy_htod: FnCuMemcpyHtoD,
    cu_memcpy_dtoh: FnCuMemcpyDtoH,
    cu_launch_kernel: FnCuLaunchKernel,
    ctx: CuContext,
    _module: CuModule,
    f_axpy: CuFunction,
    f_relu: CuFunction,
    f_dot_partial: CuFunction,
    f_matvec: CuFunction,
    f_matmul_tiled: CuFunction,
    device_name: String,
    // Persistent capacity-growing scratch buffers to amortize device allocation overhead
    buf_a: (CuDevicePtr, usize),
    buf_b: (CuDevicePtr, usize),
    buf_c: (CuDevicePtr, usize),
}

unsafe impl Send for CudaApi {}

impl CudaApi {
    unsafe fn load_sym<T>(lib: *mut c_void, name: &[u8]) -> Result<T, String> {
        let sym = dlsym(lib, name.as_ptr() as *const c_char);
        if sym.is_null() {
            return Err(format!(
                "missing CUDA symbol: {}",
                CStr::from_bytes_with_nul(name).unwrap().to_string_lossy()
            ));
        }
        Ok(std::mem::transmute_copy(&sym))
    }

    fn init() -> Result<Self, String> {
        unsafe {
            let lib = dlopen(b"libcuda.so.1\0".as_ptr() as *const c_char, RTLD_LAZY | RTLD_LOCAL);
            if lib.is_null() {
                return Err("failed to dlopen libcuda.so.1".to_string());
            }

            let cu_init: FnCuInit = Self::load_sym(lib, b"cuInit\0")?;
            let cu_device_get_count: FnCuDeviceGetCount = Self::load_sym(lib, b"cuDeviceGetCount\0")?;
            let cu_device_get: FnCuDeviceGet = Self::load_sym(lib, b"cuDeviceGet\0")?;
            let cu_device_get_name: FnCuDeviceGetName = Self::load_sym(lib, b"cuDeviceGetName\0")?;
            let cu_ctx_create: FnCuCtxCreate = Self::load_sym(lib, b"cuCtxCreate_v2\0")?;
            let cu_ctx_set_current: FnCuCtxSetCurrent = Self::load_sym(lib, b"cuCtxSetCurrent\0")?;
            let cu_ctx_synchronize: FnCuCtxSynchronize = Self::load_sym(lib, b"cuCtxSynchronize\0")?;
            let cu_module_load_data: FnCuModuleLoadData = Self::load_sym(lib, b"cuModuleLoadData\0")?;
            let cu_module_get_function: FnCuModuleGetFunction =
                Self::load_sym(lib, b"cuModuleGetFunction\0")?;
            let cu_mem_alloc: FnCuMemAlloc = Self::load_sym(lib, b"cuMemAlloc_v2\0")?;
            let cu_mem_free: FnCuMemFree = Self::load_sym(lib, b"cuMemFree_v2\0")?;
            let cu_memcpy_htod: FnCuMemcpyHtoD = Self::load_sym(lib, b"cuMemcpyHtoD_v2\0")?;
            let cu_memcpy_dtoh: FnCuMemcpyDtoH = Self::load_sym(lib, b"cuMemcpyDtoH_v2\0")?;
            let cu_launch_kernel: FnCuLaunchKernel = Self::load_sym(lib, b"cuLaunchKernel\0")?;

            let check = |res: CuResult, ctx: &str| -> Result<(), String> {
                if res == CUDA_SUCCESS {
                    Ok(())
                } else {
                    Err(format!("CUDA error {res} in {ctx}"))
                }
            };

            check(cu_init(0), "cuInit")?;
            let mut count: c_int = 0;
            check(cu_device_get_count(&mut count), "cuDeviceGetCount")?;
            if count <= 0 {
                return Err("no CUDA devices available".to_string());
            }
            let mut dev: CuDevice = 0;
            check(cu_device_get(&mut dev, 0), "cuDeviceGet")?;
            let mut name_buf = [0i8; 256];
            check(cu_device_get_name(name_buf.as_mut_ptr(), 256, dev), "cuDeviceGetName")?;
            let device_name = CStr::from_ptr(name_buf.as_ptr()).to_string_lossy().into_owned();

            let mut ctx: CuContext = ptr::null_mut();
            check(cu_ctx_create(&mut ctx, 0, dev), "cuCtxCreate_v2")?;

            let mut ptx_bytes = PTX_SOURCE.as_bytes().to_vec();
            ptx_bytes.push(0);
            let mut module: CuModule = ptr::null_mut();
            check(
                cu_module_load_data(&mut module, ptx_bytes.as_ptr() as *const c_void),
                "cuModuleLoadData",
            )?;

            let mut f_axpy = ptr::null_mut();
            let mut f_relu = ptr::null_mut();
            let mut f_dot_partial = ptr::null_mut();
            let mut f_matvec = ptr::null_mut();
            let mut f_matmul_tiled = ptr::null_mut();
            check(
                cu_module_get_function(&mut f_axpy, module, b"k_axpy\0".as_ptr() as *const c_char),
                "get k_axpy",
            )?;
            check(
                cu_module_get_function(&mut f_relu, module, b"k_relu\0".as_ptr() as *const c_char),
                "get k_relu",
            )?;
            check(
                cu_module_get_function(
                    &mut f_dot_partial,
                    module,
                    b"k_dot_partial\0".as_ptr() as *const c_char,
                ),
                "get k_dot_partial",
            )?;
            check(
                cu_module_get_function(&mut f_matvec, module, b"k_matvec\0".as_ptr() as *const c_char),
                "get k_matvec",
            )?;
            check(
                cu_module_get_function(
                    &mut f_matmul_tiled,
                    module,
                    b"k_matmul_tiled\0".as_ptr() as *const c_char,
                ),
                "get k_matmul_tiled",
            )?;

            Ok(Self {
                _lib: lib,
                cu_ctx_set_current,
                cu_ctx_synchronize,
                cu_mem_alloc,
                cu_mem_free,
                cu_memcpy_htod,
                cu_memcpy_dtoh,
                cu_launch_kernel,
                ctx,
                _module: module,
                f_axpy,
                f_relu,
                f_dot_partial,
                f_matvec,
                f_matmul_tiled,
                device_name,
                buf_a: (0, 0),
                buf_b: (0, 0),
                buf_c: (0, 0),
            })
        }
    }

    unsafe fn ensure_buf(
        cu_mem_alloc: FnCuMemAlloc,
        cu_mem_free: FnCuMemFree,
        slot: &mut (CuDevicePtr, usize),
        bytes: usize,
    ) -> CuDevicePtr {
        let need = bytes.max(4);
        if slot.1 < need {
            if slot.0 != 0 {
                let _ = cu_mem_free(slot.0);
            }
            let mut ptr: CuDevicePtr = 0;
            let rc = cu_mem_alloc(&mut ptr, need);
            assert_eq!(rc, CUDA_SUCCESS, "cuMemAlloc failed ({rc}) for {need} bytes");
            *slot = (ptr, need);
        }
        slot.0
    }

    fn bind(&self) {
        let rc = unsafe { (self.cu_ctx_set_current)(self.ctx) };
        assert_eq!(rc, CUDA_SUCCESS, "cuCtxSetCurrent failed ({rc})");
    }
}

fn runtime() -> &'static Mutex<Result<CudaApi, String>> {
    static RT: OnceLock<Mutex<Result<CudaApi, String>>> = OnceLock::new();
    RT.get_or_init(|| Mutex::new(CudaApi::init()))
}

pub fn is_available() -> bool {
    runtime().lock().unwrap().is_ok()
}

pub fn device_name() -> Option<String> {
    runtime()
        .lock()
        .unwrap()
        .as_ref()
        .ok()
        .map(|rt| rt.device_name.clone())
}

#[inline(never)]
pub fn dot(a: &Tensor, b: &Tensor) -> f32 {
    assert_eq!(a.len(), b.len(), "cuda::dot: mismatched lengths");
    let n = a.len();
    if n == 0 {
        return 0.0;
    }
    let mut guard = runtime().lock().unwrap();
    let rt = guard.as_mut().expect("CUDA runtime unavailable");
    rt.bind();
    unsafe {
        let bytes = n * 4;
        let blocks = ((n + 255) / 256).min(1024) as u32;
        let da = CudaApi::ensure_buf(rt.cu_mem_alloc, rt.cu_mem_free, &mut rt.buf_a, bytes);
        let db = CudaApi::ensure_buf(rt.cu_mem_alloc, rt.cu_mem_free, &mut rt.buf_b, bytes);
        let dc = CudaApi::ensure_buf(
            rt.cu_mem_alloc,
            rt.cu_mem_free,
            &mut rt.buf_c,
            (blocks as usize) * 4,
        );
        assert_eq!(
            (rt.cu_memcpy_htod)(da, a.as_slice().as_ptr() as *const c_void, bytes),
            CUDA_SUCCESS
        );
        assert_eq!(
            (rt.cu_memcpy_htod)(db, b.as_slice().as_ptr() as *const c_void, bytes),
            CUDA_SUCCESS
        );
        let mut p_a = da;
        let mut p_b = db;
        let mut p_c = dc;
        let mut p_n = n as u32;
        let mut params: [*mut c_void; 4] = [
            &mut p_a as *mut _ as *mut c_void,
            &mut p_b as *mut _ as *mut c_void,
            &mut p_c as *mut _ as *mut c_void,
            &mut p_n as *mut _ as *mut c_void,
        ];
        assert_eq!(
            (rt.cu_launch_kernel)(
                rt.f_dot_partial,
                blocks,
                1,
                1,
                256,
                1,
                1,
                0,
                ptr::null_mut(),
                params.as_mut_ptr(),
                ptr::null_mut(),
            ),
            CUDA_SUCCESS
        );
        assert_eq!((rt.cu_ctx_synchronize)(), CUDA_SUCCESS);
        let mut partials = vec![0.0_f32; blocks as usize];
        assert_eq!(
            (rt.cu_memcpy_dtoh)(
                partials.as_mut_ptr() as *mut c_void,
                dc,
                (blocks as usize) * 4,
            ),
            CUDA_SUCCESS
        );
        partials.into_iter().map(|v| v as f64).sum::<f64>() as f32
    }
}

#[inline(never)]
pub fn axpy(alpha: f32, x: &Tensor, y: &mut Tensor) {
    assert_eq!(x.len(), y.len(), "cuda::axpy: mismatched lengths");
    let n = x.len();
    if n == 0 {
        return;
    }
    let mut guard = runtime().lock().unwrap();
    let rt = guard.as_mut().expect("CUDA runtime unavailable");
    rt.bind();
    unsafe {
        let bytes = n * 4;
        let dx = CudaApi::ensure_buf(rt.cu_mem_alloc, rt.cu_mem_free, &mut rt.buf_a, bytes);
        let dy = CudaApi::ensure_buf(rt.cu_mem_alloc, rt.cu_mem_free, &mut rt.buf_b, bytes);
        assert_eq!(
            (rt.cu_memcpy_htod)(dx, x.as_slice().as_ptr() as *const c_void, bytes),
            CUDA_SUCCESS
        );
        assert_eq!(
            (rt.cu_memcpy_htod)(dy, y.as_slice().as_ptr() as *const c_void, bytes),
            CUDA_SUCCESS
        );
        let mut p_x = dx;
        let mut p_y = dy;
        let mut p_alpha = alpha;
        let mut p_n = n as u32;
        let mut params: [*mut c_void; 4] = [
            &mut p_x as *mut _ as *mut c_void,
            &mut p_y as *mut _ as *mut c_void,
            &mut p_alpha as *mut _ as *mut c_void,
            &mut p_n as *mut _ as *mut c_void,
        ];
        let blocks = ((n + 255) / 256) as u32;
        assert_eq!(
            (rt.cu_launch_kernel)(
                rt.f_axpy,
                blocks,
                1,
                1,
                256,
                1,
                1,
                0,
                ptr::null_mut(),
                params.as_mut_ptr(),
                ptr::null_mut(),
            ),
            CUDA_SUCCESS
        );
        assert_eq!((rt.cu_ctx_synchronize)(), CUDA_SUCCESS);
        assert_eq!(
            (rt.cu_memcpy_dtoh)(y.as_mut_slice().as_mut_ptr() as *mut c_void, dy, bytes),
            CUDA_SUCCESS
        );
    }
}

#[inline(never)]
pub fn relu(x: &Tensor) -> Tensor {
    let n = x.len();
    let mut out = Tensor::zeros(x.shape());
    if n == 0 {
        return out;
    }
    let mut guard = runtime().lock().unwrap();
    let rt = guard.as_mut().expect("CUDA runtime unavailable");
    rt.bind();
    unsafe {
        let bytes = n * 4;
        let dx = CudaApi::ensure_buf(rt.cu_mem_alloc, rt.cu_mem_free, &mut rt.buf_a, bytes);
        let do_ptr = CudaApi::ensure_buf(rt.cu_mem_alloc, rt.cu_mem_free, &mut rt.buf_b, bytes);
        assert_eq!(
            (rt.cu_memcpy_htod)(dx, x.as_slice().as_ptr() as *const c_void, bytes),
            CUDA_SUCCESS
        );
        let mut p_x = dx;
        let mut p_o = do_ptr;
        let mut p_n = n as u32;
        let mut params: [*mut c_void; 3] = [
            &mut p_x as *mut _ as *mut c_void,
            &mut p_o as *mut _ as *mut c_void,
            &mut p_n as *mut _ as *mut c_void,
        ];
        let blocks = ((n + 255) / 256) as u32;
        assert_eq!(
            (rt.cu_launch_kernel)(
                rt.f_relu,
                blocks,
                1,
                1,
                256,
                1,
                1,
                0,
                ptr::null_mut(),
                params.as_mut_ptr(),
                ptr::null_mut(),
            ),
            CUDA_SUCCESS
        );
        assert_eq!((rt.cu_ctx_synchronize)(), CUDA_SUCCESS);
        assert_eq!(
            (rt.cu_memcpy_dtoh)(out.as_mut_slice().as_mut_ptr() as *mut c_void, do_ptr, bytes),
            CUDA_SUCCESS
        );
    }
    out
}

#[inline(never)]
pub fn matvec(a: &Tensor, x: &Tensor) -> Tensor {
    let (m, k) = a.dims_2d();
    assert_eq!(x.len(), k, "cuda::matvec: vector length {} != cols {}", x.len(), k);
    let mut y = Tensor::zeros(&[m]);
    if m == 0 || k == 0 {
        return y;
    }
    let mut guard = runtime().lock().unwrap();
    let rt = guard.as_mut().expect("CUDA runtime unavailable");
    rt.bind();
    unsafe {
        let da = CudaApi::ensure_buf(rt.cu_mem_alloc, rt.cu_mem_free, &mut rt.buf_a, m * k * 4);
        let dx = CudaApi::ensure_buf(rt.cu_mem_alloc, rt.cu_mem_free, &mut rt.buf_b, k * 4);
        let dy = CudaApi::ensure_buf(rt.cu_mem_alloc, rt.cu_mem_free, &mut rt.buf_c, m * 4);
        assert_eq!(
            (rt.cu_memcpy_htod)(da, a.as_slice().as_ptr() as *const c_void, m * k * 4),
            CUDA_SUCCESS
        );
        assert_eq!(
            (rt.cu_memcpy_htod)(dx, x.as_slice().as_ptr() as *const c_void, k * 4),
            CUDA_SUCCESS
        );
        let mut p_a = da;
        let mut p_x = dx;
        let mut p_y = dy;
        let mut p_m = m as u32;
        let mut p_k = k as u32;
        let mut params: [*mut c_void; 5] = [
            &mut p_a as *mut _ as *mut c_void,
            &mut p_x as *mut _ as *mut c_void,
            &mut p_y as *mut _ as *mut c_void,
            &mut p_m as *mut _ as *mut c_void,
            &mut p_k as *mut _ as *mut c_void,
        ];
        assert_eq!(
            (rt.cu_launch_kernel)(
                rt.f_matvec,
                m as u32,
                1,
                1,
                256,
                1,
                1,
                0,
                ptr::null_mut(),
                params.as_mut_ptr(),
                ptr::null_mut(),
            ),
            CUDA_SUCCESS
        );
        assert_eq!((rt.cu_ctx_synchronize)(), CUDA_SUCCESS);
        assert_eq!(
            (rt.cu_memcpy_dtoh)(y.as_mut_slice().as_mut_ptr() as *mut c_void, dy, m * 4),
            CUDA_SUCCESS
        );
    }
    y
}

#[inline(never)]
pub fn matmul(a: &Tensor, b: &Tensor) -> Tensor {
    let (m, k) = a.dims_2d();
    let (bk, n) = b.dims_2d();
    assert_eq!(k, bk, "cuda::matmul: inner dims {} != {}", k, bk);
    let mut c = Tensor::zeros(&[m, n]);
    if m == 0 || k == 0 || n == 0 {
        return c;
    }
    let mut guard = runtime().lock().unwrap();
    let rt = guard.as_mut().expect("CUDA runtime unavailable");
    rt.bind();
    unsafe {
        let da = CudaApi::ensure_buf(rt.cu_mem_alloc, rt.cu_mem_free, &mut rt.buf_a, m * k * 4);
        let db = CudaApi::ensure_buf(rt.cu_mem_alloc, rt.cu_mem_free, &mut rt.buf_b, k * n * 4);
        let dc = CudaApi::ensure_buf(rt.cu_mem_alloc, rt.cu_mem_free, &mut rt.buf_c, m * n * 4);
        assert_eq!(
            (rt.cu_memcpy_htod)(da, a.as_slice().as_ptr() as *const c_void, m * k * 4),
            CUDA_SUCCESS
        );
        assert_eq!(
            (rt.cu_memcpy_htod)(db, b.as_slice().as_ptr() as *const c_void, k * n * 4),
            CUDA_SUCCESS
        );
        let mut p_a = da;
        let mut p_b = db;
        let mut p_c = dc;
        let mut p_m = m as u32;
        let mut p_k = k as u32;
        let mut p_n = n as u32;
        let mut params: [*mut c_void; 6] = [
            &mut p_a as *mut _ as *mut c_void,
            &mut p_b as *mut _ as *mut c_void,
            &mut p_c as *mut _ as *mut c_void,
            &mut p_m as *mut _ as *mut c_void,
            &mut p_k as *mut _ as *mut c_void,
            &mut p_n as *mut _ as *mut c_void,
        ];
        let gx = ((n + 15) / 16) as u32;
        let gy = ((m + 15) / 16) as u32;
        assert_eq!(
            (rt.cu_launch_kernel)(
                rt.f_matmul_tiled,
                gx,
                gy,
                1,
                16,
                16,
                1,
                0,
                ptr::null_mut(),
                params.as_mut_ptr(),
                ptr::null_mut(),
            ),
            CUDA_SUCCESS
        );
        assert_eq!((rt.cu_ctx_synchronize)(), CUDA_SUCCESS);
        assert_eq!(
            (rt.cu_memcpy_dtoh)(c.as_mut_slice().as_mut_ptr() as *mut c_void, dc, m * n * 4),
            CUDA_SUCCESS
        );
    }
    c
}

/// Benchmarks GPU `matmul` separating pure device kernel latency (VRAM-resident)
/// from end-to-end PCIe (HtoD + kernel + DtoH) latency.
pub fn bench_matmul_breakdown(
    a: &Tensor,
    b: &Tensor,
    warmup: usize,
    iters: usize,
) -> (Vec<f64>, Vec<f64>) {
    let (m, k) = a.dims_2d();
    let (bk, n) = b.dims_2d();
    assert_eq!(k, bk);
    let mut c = Tensor::zeros(&[m, n]);
    let mut guard = runtime().lock().unwrap();
    let rt = guard.as_mut().expect("CUDA runtime unavailable");
    rt.bind();

    let mut kernel_ms = Vec::with_capacity(iters);
    let mut pcie_e2e_ms = Vec::with_capacity(iters);

    unsafe {
        let da = CudaApi::ensure_buf(rt.cu_mem_alloc, rt.cu_mem_free, &mut rt.buf_a, m * k * 4);
        let db = CudaApi::ensure_buf(rt.cu_mem_alloc, rt.cu_mem_free, &mut rt.buf_b, k * n * 4);
        let dc = CudaApi::ensure_buf(rt.cu_mem_alloc, rt.cu_mem_free, &mut rt.buf_c, m * n * 4);
        assert_eq!(
            (rt.cu_memcpy_htod)(da, a.as_slice().as_ptr() as *const c_void, m * k * 4),
            CUDA_SUCCESS
        );
        assert_eq!(
            (rt.cu_memcpy_htod)(db, b.as_slice().as_ptr() as *const c_void, k * n * 4),
            CUDA_SUCCESS
        );

        let mut p_a = da;
        let mut p_b = db;
        let mut p_c = dc;
        let mut p_m = m as u32;
        let mut p_k = k as u32;
        let mut p_n = n as u32;
        let mut params: [*mut c_void; 6] = [
            &mut p_a as *mut _ as *mut c_void,
            &mut p_b as *mut _ as *mut c_void,
            &mut p_c as *mut _ as *mut c_void,
            &mut p_m as *mut _ as *mut c_void,
            &mut p_k as *mut _ as *mut c_void,
            &mut p_n as *mut _ as *mut c_void,
        ];
        let gx = ((n + 15) / 16) as u32;
        let gy = ((m + 15) / 16) as u32;

        // 1. Kernel-only (VRAM-resident)
        for _ in 0..warmup {
            assert_eq!(
                (rt.cu_launch_kernel)(
                    rt.f_matmul_tiled, gx, gy, 1, 16, 16, 1, 0,
                    ptr::null_mut(), params.as_mut_ptr(), ptr::null_mut()
                ),
                CUDA_SUCCESS
            );
            assert_eq!((rt.cu_ctx_synchronize)(), CUDA_SUCCESS);
        }
        for _ in 0..iters {
            let t0 = Instant::now();
            assert_eq!(
                (rt.cu_launch_kernel)(
                    rt.f_matmul_tiled, gx, gy, 1, 16, 16, 1, 0,
                    ptr::null_mut(), params.as_mut_ptr(), ptr::null_mut()
                ),
                CUDA_SUCCESS
            );
            assert_eq!((rt.cu_ctx_synchronize)(), CUDA_SUCCESS);
            kernel_ms.push(t0.elapsed().as_secs_f64() * 1_000.0);
        }

        // 2. End-to-end PCIe (HtoD + kernel + DtoH)
        for _ in 0..warmup {
            assert_eq!(
                (rt.cu_memcpy_htod)(da, a.as_slice().as_ptr() as *const c_void, m * k * 4),
                CUDA_SUCCESS
            );
            assert_eq!(
                (rt.cu_memcpy_htod)(db, b.as_slice().as_ptr() as *const c_void, k * n * 4),
                CUDA_SUCCESS
            );
            assert_eq!(
                (rt.cu_launch_kernel)(
                    rt.f_matmul_tiled, gx, gy, 1, 16, 16, 1, 0,
                    ptr::null_mut(), params.as_mut_ptr(), ptr::null_mut()
                ),
                CUDA_SUCCESS
            );
            assert_eq!((rt.cu_ctx_synchronize)(), CUDA_SUCCESS);
            assert_eq!(
                (rt.cu_memcpy_dtoh)(c.as_mut_slice().as_mut_ptr() as *mut c_void, dc, m * n * 4),
                CUDA_SUCCESS
            );
        }
        for _ in 0..iters {
            let t0 = Instant::now();
            assert_eq!(
                (rt.cu_memcpy_htod)(da, a.as_slice().as_ptr() as *const c_void, m * k * 4),
                CUDA_SUCCESS
            );
            assert_eq!(
                (rt.cu_memcpy_htod)(db, b.as_slice().as_ptr() as *const c_void, k * n * 4),
                CUDA_SUCCESS
            );
            assert_eq!(
                (rt.cu_launch_kernel)(
                    rt.f_matmul_tiled, gx, gy, 1, 16, 16, 1, 0,
                    ptr::null_mut(), params.as_mut_ptr(), ptr::null_mut()
                ),
                CUDA_SUCCESS
            );
            assert_eq!((rt.cu_ctx_synchronize)(), CUDA_SUCCESS);
            assert_eq!(
                (rt.cu_memcpy_dtoh)(c.as_mut_slice().as_mut_ptr() as *mut c_void, dc, m * n * 4),
                CUDA_SUCCESS
            );
            pcie_e2e_ms.push(t0.elapsed().as_secs_f64() * 1_000.0);
        }
    }

    (kernel_ms, pcie_e2e_ms)
}
