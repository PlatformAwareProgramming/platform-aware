//! Function multi-versioning comparison engine using the `multiversion` crate (v0.8.1).
//!
//! Uses idiomatic `#[multiversion::multiversion]` with `match_target!` on free functions
//! (since `multiversion` does not support `self`/`Self` methods), sharing the exact same
//! `kernels.rs` implementations as `platform-aware`, `static-cfg`, and `ad-hoc`.
//! Because `multiversion` only models CPU ISA target features, multicore dispatch
//! (`worker_count() >= 2`) must still be layered manually inside each operation.

use multiversion::multiversion;
use multiversion::target::match_target;

use crate::kernels::{avx2, avx512, multicore, scalar, sse42, worker_count};
use crate::tensor::Tensor;

#[multiversion(targets("x86_64+avx512f", "x86_64+avx2+fma", "x86_64+sse4.2"))]
fn active_backend_simd_code() -> u8 {
    match_target! {
        "x86_64+avx512f" => 3,
        "x86_64+avx2+fma" => 2,
        "x86_64+sse4.2" => 1,
        _ => 0,
    }
}

fn active_backend_simd_fn() -> &'static str {
    match active_backend_simd_code() {
        3 => "avx512",
        2 => "avx2",
        1 => "sse42",
        _ => "scalar",
    }
}

#[multiversion(targets("x86_64+avx512f", "x86_64+avx2+fma", "x86_64+sse4.2"))]
#[inline(never)]
fn dispatch_probe_fn(x: u64) -> u64 {
    x.wrapping_add(1)
}

#[multiversion(targets("x86_64+avx512f", "x86_64+avx2+fma", "x86_64+sse4.2"))]
fn dot_simd_fn(a: &Tensor, b: &Tensor) -> f32 {
    match_target! {
        "x86_64+avx512f" => avx512::dot(a, b),
        "x86_64+avx2+fma" => avx2::dot(a, b),
        "x86_64+sse4.2" => sse42::dot(a, b),
        _ => scalar::dot(a, b),
    }
}

#[multiversion(targets("x86_64+avx512f", "x86_64+avx2+fma", "x86_64+sse4.2"))]
fn axpy_simd_fn(alpha: f32, x: &Tensor, y: &mut Tensor) {
    match_target! {
        "x86_64+avx512f" => avx512::axpy(alpha, x, y),
        "x86_64+avx2+fma" => avx2::axpy(alpha, x, y),
        "x86_64+sse4.2" => sse42::axpy(alpha, x, y),
        _ => scalar::axpy(alpha, x, y),
    }
}

#[multiversion(targets("x86_64+avx512f", "x86_64+avx2+fma", "x86_64+sse4.2"))]
fn relu_simd_fn(x: &Tensor) -> Tensor {
    match_target! {
        "x86_64+avx512f" => avx512::relu(x),
        "x86_64+avx2+fma" => avx2::relu(x),
        "x86_64+sse4.2" => sse42::relu(x),
        _ => scalar::relu(x),
    }
}

#[multiversion(targets("x86_64+avx512f", "x86_64+avx2+fma", "x86_64+sse4.2"))]
fn matvec_simd_fn(a: &Tensor, x: &Tensor) -> Tensor {
    match_target! {
        "x86_64+avx512f" => avx512::matvec(a, x),
        "x86_64+avx2+fma" => avx2::matvec(a, x),
        "x86_64+sse4.2" => sse42::matvec(a, x),
        _ => scalar::matvec(a, x),
    }
}

#[multiversion(targets("x86_64+avx512f", "x86_64+avx2+fma", "x86_64+sse4.2"))]
fn matmul_simd_fn(a: &Tensor, b: &Tensor) -> Tensor {
    match_target! {
        "x86_64+avx512f" => avx512::matmul(a, b),
        "x86_64+avx2+fma" => avx2::matmul(a, b),
        "x86_64+sse4.2" => sse42::matmul(a, b),
        _ => scalar::matmul(a, b),
    }
}

#[multiversion(targets("x86_64+avx512f", "x86_64+avx2+fma", "x86_64+sse4.2"))]
fn dot_hybrid_fn(a: &Tensor, b: &Tensor) -> f32 {
    let multi = worker_count() >= 2;
    match_target! {
        "x86_64+avx512f" => if multi { multicore::avx512_mt::dot(a, b) } else { avx512::dot(a, b) },
        "x86_64+avx2+fma" => if multi { multicore::avx2_mt::dot(a, b) } else { avx2::dot(a, b) },
        "x86_64+sse4.2" => if multi { multicore::scalar_mt::dot(a, b) } else { sse42::dot(a, b) },
        _ => if multi { multicore::scalar_mt::dot(a, b) } else { scalar::dot(a, b) },
    }
}

#[multiversion(targets("x86_64+avx512f", "x86_64+avx2+fma", "x86_64+sse4.2"))]
fn axpy_hybrid_fn(alpha: f32, x: &Tensor, y: &mut Tensor) {
    let multi = worker_count() >= 2;
    match_target! {
        "x86_64+avx512f" => if multi { multicore::avx512_mt::axpy(alpha, x, y) } else { avx512::axpy(alpha, x, y) },
        "x86_64+avx2+fma" => if multi { multicore::avx2_mt::axpy(alpha, x, y) } else { avx2::axpy(alpha, x, y) },
        "x86_64+sse4.2" => if multi { multicore::scalar_mt::axpy(alpha, x, y) } else { sse42::axpy(alpha, x, y) },
        _ => if multi { multicore::scalar_mt::axpy(alpha, x, y) } else { scalar::axpy(alpha, x, y) },
    }
}

#[multiversion(targets("x86_64+avx512f", "x86_64+avx2+fma", "x86_64+sse4.2"))]
fn relu_hybrid_fn(x: &Tensor) -> Tensor {
    let multi = worker_count() >= 2;
    match_target! {
        "x86_64+avx512f" => if multi { multicore::avx512_mt::relu(x) } else { avx512::relu(x) },
        "x86_64+avx2+fma" => if multi { multicore::avx2_mt::relu(x) } else { avx2::relu(x) },
        "x86_64+sse4.2" => if multi { multicore::scalar_mt::relu(x) } else { sse42::relu(x) },
        _ => if multi { multicore::scalar_mt::relu(x) } else { scalar::relu(x) },
    }
}

#[multiversion(targets("x86_64+avx512f", "x86_64+avx2+fma", "x86_64+sse4.2"))]
fn matvec_hybrid_fn(a: &Tensor, x: &Tensor) -> Tensor {
    let multi = worker_count() >= 2;
    match_target! {
        "x86_64+avx512f" => if multi { multicore::avx512_mt::matvec(a, x) } else { avx512::matvec(a, x) },
        "x86_64+avx2+fma" => if multi { multicore::avx2_mt::matvec(a, x) } else { avx2::matvec(a, x) },
        "x86_64+sse4.2" => if multi { multicore::scalar_mt::matvec(a, x) } else { sse42::matvec(a, x) },
        _ => if multi { multicore::scalar_mt::matvec(a, x) } else { scalar::matvec(a, x) },
    }
}

#[multiversion(targets("x86_64+avx512f", "x86_64+avx2+fma", "x86_64+sse4.2"))]
fn matmul_hybrid_fn(a: &Tensor, b: &Tensor) -> Tensor {
    let multi = worker_count() >= 2;
    match_target! {
        "x86_64+avx512f" => if multi { multicore::avx512_mt::matmul(a, b) } else { avx512::matmul(a, b) },
        "x86_64+avx2+fma" => if multi { multicore::avx2_mt::matmul(a, b) } else { avx2::matmul(a, b) },
        "x86_64+sse4.2" => if multi { multicore::scalar_mt::matmul(a, b) } else { sse42::matmul(a, b) },
        _ => if multi { multicore::scalar_mt::matmul(a, b) } else { scalar::matmul(a, b) },
    }
}

pub struct MultiversionEngine;

impl MultiversionEngine {
    pub fn active_backend() -> &'static str {
        active_backend_simd_fn()
    }

    pub fn active_backend_hybrid() -> &'static str {
        let multi = worker_count() >= 2;
        match (multi, active_backend_simd_fn()) {
            (true, "avx512") => "multicore_avx512",
            (true, "avx2") => "multicore_avx2",
            (true, _) => "multicore_scalar",
            (false, b) => b,
        }
    }

    pub fn dispatch_probe(x: u64) -> u64 {
        dispatch_probe_fn(x)
    }

    pub fn dot_simd(a: &Tensor, b: &Tensor) -> f32 {
        dot_simd_fn(a, b)
    }

    pub fn axpy_simd(alpha: f32, x: &Tensor, y: &mut Tensor) {
        axpy_simd_fn(alpha, x, y);
    }

    pub fn relu_simd(x: &Tensor) -> Tensor {
        relu_simd_fn(x)
    }

    pub fn matvec_simd(a: &Tensor, x: &Tensor) -> Tensor {
        matvec_simd_fn(a, x)
    }

    pub fn matmul_simd(a: &Tensor, b: &Tensor) -> Tensor {
        matmul_simd_fn(a, b)
    }

    pub fn dot(a: &Tensor, b: &Tensor) -> f32 {
        dot_hybrid_fn(a, b)
    }

    pub fn axpy(alpha: f32, x: &Tensor, y: &mut Tensor) {
        axpy_hybrid_fn(alpha, x, y);
    }

    pub fn relu(x: &Tensor) -> Tensor {
        relu_hybrid_fn(x)
    }

    pub fn matvec(a: &Tensor, x: &Tensor) -> Tensor {
        matvec_hybrid_fn(a, x)
    }

    pub fn matmul(a: &Tensor, b: &Tensor) -> Tensor {
        matmul_hybrid_fn(a, b)
    }
}
