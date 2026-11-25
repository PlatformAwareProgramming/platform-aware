//! Idiomatic zero-dependency dynamic dispatch baseline using a trait object
//! cached once in `OnceLock<&'static dyn KernelBackend>`.
//!
//! Unlike naive `AdHocRuntimeEngine` (which repeats `is_x86_feature_detected!` inside every
//! operation), this pattern isolates each backend in its own type/module and resolves the
//! active vtable once at startup, preserving open-closed extensibility on axis P without
//! external crates — though without `platform-aware`'s declarative subtyping lattice.

use std::sync::OnceLock;

use crate::kernels::{avx2, avx512, multicore, scalar, sse42, worker_count};
use crate::tensor::Tensor;

pub trait KernelBackend: Sync {
    fn name(&self) -> &'static str;
    fn dispatch_probe(&self, x: u64) -> u64;
    fn dot(&self, a: &Tensor, b: &Tensor) -> f32;
    fn axpy(&self, alpha: f32, x: &Tensor, y: &mut Tensor);
    fn relu(&self, x: &Tensor) -> Tensor;
    fn matvec(&self, a: &Tensor, x: &Tensor) -> Tensor;
    fn matmul(&self, a: &Tensor, b: &Tensor) -> Tensor;
}

struct ScalarBackend;
struct Sse42Backend;
struct Avx2Backend;
struct Avx512Backend;
struct McScalarBackend;
struct McAvx2Backend;
struct McAvx512Backend;

impl KernelBackend for ScalarBackend {
    fn name(&self) -> &'static str { "scalar" }
    #[inline(never)]
    fn dispatch_probe(&self, x: u64) -> u64 { x.wrapping_add(1) }
    fn dot(&self, a: &Tensor, b: &Tensor) -> f32 { scalar::dot(a, b) }
    fn axpy(&self, alpha: f32, x: &Tensor, y: &mut Tensor) { scalar::axpy(alpha, x, y); }
    fn relu(&self, x: &Tensor) -> Tensor { scalar::relu(x) }
    fn matvec(&self, a: &Tensor, x: &Tensor) -> Tensor { scalar::matvec(a, x) }
    fn matmul(&self, a: &Tensor, b: &Tensor) -> Tensor { scalar::matmul(a, b) }
}

impl KernelBackend for Sse42Backend {
    fn name(&self) -> &'static str { "sse42" }
    #[inline(never)]
    fn dispatch_probe(&self, x: u64) -> u64 { x.wrapping_add(1) }
    fn dot(&self, a: &Tensor, b: &Tensor) -> f32 { sse42::dot(a, b) }
    fn axpy(&self, alpha: f32, x: &Tensor, y: &mut Tensor) { sse42::axpy(alpha, x, y); }
    fn relu(&self, x: &Tensor) -> Tensor { sse42::relu(x) }
    fn matvec(&self, a: &Tensor, x: &Tensor) -> Tensor { sse42::matvec(a, x) }
    fn matmul(&self, a: &Tensor, b: &Tensor) -> Tensor { sse42::matmul(a, b) }
}

impl KernelBackend for Avx2Backend {
    fn name(&self) -> &'static str { "avx2" }
    #[inline(never)]
    fn dispatch_probe(&self, x: u64) -> u64 { x.wrapping_add(1) }
    fn dot(&self, a: &Tensor, b: &Tensor) -> f32 { avx2::dot(a, b) }
    fn axpy(&self, alpha: f32, x: &Tensor, y: &mut Tensor) { avx2::axpy(alpha, x, y); }
    fn relu(&self, x: &Tensor) -> Tensor { avx2::relu(x) }
    fn matvec(&self, a: &Tensor, x: &Tensor) -> Tensor { avx2::matvec(a, x) }
    fn matmul(&self, a: &Tensor, b: &Tensor) -> Tensor { avx2::matmul(a, b) }
}

impl KernelBackend for Avx512Backend {
    fn name(&self) -> &'static str { "avx512" }
    #[inline(never)]
    fn dispatch_probe(&self, x: u64) -> u64 { x.wrapping_add(1) }
    fn dot(&self, a: &Tensor, b: &Tensor) -> f32 { avx512::dot(a, b) }
    fn axpy(&self, alpha: f32, x: &Tensor, y: &mut Tensor) { avx512::axpy(alpha, x, y); }
    fn relu(&self, x: &Tensor) -> Tensor { avx512::relu(x) }
    fn matvec(&self, a: &Tensor, x: &Tensor) -> Tensor { avx512::matvec(a, x) }
    fn matmul(&self, a: &Tensor, b: &Tensor) -> Tensor { avx512::matmul(a, b) }
}

impl KernelBackend for McScalarBackend {
    fn name(&self) -> &'static str { "multicore_scalar" }
    #[inline(never)]
    fn dispatch_probe(&self, x: u64) -> u64 { x.wrapping_add(1) }
    fn dot(&self, a: &Tensor, b: &Tensor) -> f32 { multicore::scalar_mt::dot(a, b) }
    fn axpy(&self, alpha: f32, x: &Tensor, y: &mut Tensor) { multicore::scalar_mt::axpy(alpha, x, y); }
    fn relu(&self, x: &Tensor) -> Tensor { multicore::scalar_mt::relu(x) }
    fn matvec(&self, a: &Tensor, x: &Tensor) -> Tensor { multicore::scalar_mt::matvec(a, x) }
    fn matmul(&self, a: &Tensor, b: &Tensor) -> Tensor { multicore::scalar_mt::matmul(a, b) }
}

impl KernelBackend for McAvx2Backend {
    fn name(&self) -> &'static str { "multicore_avx2" }
    #[inline(never)]
    fn dispatch_probe(&self, x: u64) -> u64 { x.wrapping_add(1) }
    fn dot(&self, a: &Tensor, b: &Tensor) -> f32 { multicore::avx2_mt::dot(a, b) }
    fn axpy(&self, alpha: f32, x: &Tensor, y: &mut Tensor) { multicore::avx2_mt::axpy(alpha, x, y); }
    fn relu(&self, x: &Tensor) -> Tensor { multicore::avx2_mt::relu(x) }
    fn matvec(&self, a: &Tensor, x: &Tensor) -> Tensor { multicore::avx2_mt::matvec(a, x) }
    fn matmul(&self, a: &Tensor, b: &Tensor) -> Tensor { multicore::avx2_mt::matmul(a, b) }
}

impl KernelBackend for McAvx512Backend {
    fn name(&self) -> &'static str { "multicore_avx512" }
    #[inline(never)]
    fn dispatch_probe(&self, x: u64) -> u64 { x.wrapping_add(1) }
    fn dot(&self, a: &Tensor, b: &Tensor) -> f32 { multicore::avx512_mt::dot(a, b) }
    fn axpy(&self, alpha: f32, x: &Tensor, y: &mut Tensor) { multicore::avx512_mt::axpy(alpha, x, y); }
    fn relu(&self, x: &Tensor) -> Tensor { multicore::avx512_mt::relu(x) }
    fn matvec(&self, a: &Tensor, x: &Tensor) -> Tensor { multicore::avx512_mt::matvec(a, x) }
    fn matmul(&self, a: &Tensor, b: &Tensor) -> Tensor { multicore::avx512_mt::matmul(a, b) }
}

static SCALAR: ScalarBackend = ScalarBackend;
static SSE42: Sse42Backend = Sse42Backend;
static AVX2: Avx2Backend = Avx2Backend;
static AVX512: Avx512Backend = Avx512Backend;
static MC_SCALAR: McScalarBackend = McScalarBackend;
static MC_AVX2: McAvx2Backend = McAvx2Backend;
static MC_AVX512: McAvx512Backend = McAvx512Backend;

fn select_backend() -> &'static dyn KernelBackend {
    static ACTIVE: OnceLock<&'static dyn KernelBackend> = OnceLock::new();
    *ACTIVE.get_or_init(|| {
        let multi = worker_count() >= 2;
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx512f") {
                return if multi { &MC_AVX512 } else { &AVX512 };
            }
            if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
                return if multi { &MC_AVX2 } else { &AVX2 };
            }
            if multi {
                return &MC_SCALAR;
            }
            if is_x86_feature_detected!("sse4.2") {
                return &SSE42;
            }
        }
        if multi { &MC_SCALAR } else { &SCALAR }
    })
}

pub struct AdHocIdiomaticEngine;

impl AdHocIdiomaticEngine {
    pub fn active_backend() -> &'static str { select_backend().name() }
    #[inline(never)]
    pub fn dispatch_probe(x: u64) -> u64 { select_backend().dispatch_probe(x) }
    pub fn dot(a: &Tensor, b: &Tensor) -> f32 { select_backend().dot(a, b) }
    pub fn axpy(alpha: f32, x: &Tensor, y: &mut Tensor) { select_backend().axpy(alpha, x, y); }
    pub fn relu(x: &Tensor) -> Tensor { select_backend().relu(x) }
    pub fn matvec(a: &Tensor, x: &Tensor) -> Tensor { select_backend().matvec(a, x) }
    pub fn matmul(a: &Tensor, b: &Tensor) -> Tensor { select_backend().matmul(a, b) }
}
