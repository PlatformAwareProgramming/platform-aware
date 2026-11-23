//! Static and ad-hoc comparison engines:
//!
//! 1. `StaticCfgEngine`: idiomatic Rust compile-time conditional compilation via
//!    `#[cfg(target_feature = "...")]` and Cargo feature `static-multicore`.
//!    Compiled with default flags it produces a portable binary stuck on `scalar`;
//!    compiled with `RUSTFLAGS="-C target-cpu=native"` + `--features static-multicore`
//!    it unlocks native performance at the expense of binary portability.
//! 2. `StaticDirectBackend`: zero-dispatch direct invocation of a named kernel,
//!    isolating the exact overhead of `platform-aware` dispatch (Section 5.3).
//! 3. `AdHocRuntimeEngine`: ad-hoc runtime feature detection (`is_x86_feature_detected!`)
//!    with dispatch conditionals embedded directly inside every operation (Section 2.1/3.2).

use crate::kernels::{avx2, avx512, multicore, scalar, sse42, worker_count};
use crate::tensor::Tensor;

/// Compile-time static dispatch via `#[cfg(target_feature = "...")]` and Cargo features.
pub struct StaticCfgEngine;

impl StaticCfgEngine {
    pub const fn active_backend() -> &'static str {
        #[cfg(all(feature = "static-multicore", target_feature = "avx512f"))]
        {
            "multicore_avx512"
        }
        #[cfg(all(
            feature = "static-multicore",
            not(target_feature = "avx512f"),
            target_feature = "avx2",
            target_feature = "fma"
        ))]
        {
            "multicore_avx2"
        }
        #[cfg(all(
            feature = "static-multicore",
            not(target_feature = "avx512f"),
            not(all(target_feature = "avx2", target_feature = "fma"))
        ))]
        {
            "multicore_scalar"
        }
        #[cfg(all(not(feature = "static-multicore"), target_feature = "avx512f"))]
        {
            "avx512"
        }
        #[cfg(all(
            not(feature = "static-multicore"),
            not(target_feature = "avx512f"),
            target_feature = "avx2",
            target_feature = "fma"
        ))]
        {
            "avx2"
        }
        #[cfg(all(
            not(feature = "static-multicore"),
            not(target_feature = "avx512f"),
            not(all(target_feature = "avx2", target_feature = "fma")),
            target_feature = "sse4.2"
        ))]
        {
            "sse42"
        }
        #[cfg(all(
            not(feature = "static-multicore"),
            not(target_feature = "avx512f"),
            not(all(target_feature = "avx2", target_feature = "fma")),
            not(target_feature = "sse4.2")
        ))]
        {
            "scalar"
        }
    }

    pub fn dot(a: &Tensor, b: &Tensor) -> f32 {
        #[cfg(all(feature = "static-multicore", target_feature = "avx512f"))]
        {
            multicore::avx512_mt::dot(a, b)
        }
        #[cfg(all(
            feature = "static-multicore",
            not(target_feature = "avx512f"),
            target_feature = "avx2",
            target_feature = "fma"
        ))]
        {
            multicore::avx2_mt::dot(a, b)
        }
        #[cfg(all(
            feature = "static-multicore",
            not(target_feature = "avx512f"),
            not(all(target_feature = "avx2", target_feature = "fma"))
        ))]
        {
            multicore::scalar_mt::dot(a, b)
        }
        #[cfg(all(not(feature = "static-multicore"), target_feature = "avx512f"))]
        {
            avx512::dot(a, b)
        }
        #[cfg(all(
            not(feature = "static-multicore"),
            not(target_feature = "avx512f"),
            target_feature = "avx2",
            target_feature = "fma"
        ))]
        {
            avx2::dot(a, b)
        }
        #[cfg(all(
            not(feature = "static-multicore"),
            not(target_feature = "avx512f"),
            not(all(target_feature = "avx2", target_feature = "fma")),
            target_feature = "sse4.2"
        ))]
        {
            sse42::dot(a, b)
        }
        #[cfg(all(
            not(feature = "static-multicore"),
            not(target_feature = "avx512f"),
            not(all(target_feature = "avx2", target_feature = "fma")),
            not(target_feature = "sse4.2")
        ))]
        {
            scalar::dot(a, b)
        }
    }

    pub fn axpy(alpha: f32, x: &Tensor, y: &mut Tensor) {
        #[cfg(all(feature = "static-multicore", target_feature = "avx512f"))]
        {
            multicore::avx512_mt::axpy(alpha, x, y);
        }
        #[cfg(all(
            feature = "static-multicore",
            not(target_feature = "avx512f"),
            target_feature = "avx2",
            target_feature = "fma"
        ))]
        {
            multicore::avx2_mt::axpy(alpha, x, y);
        }
        #[cfg(all(
            feature = "static-multicore",
            not(target_feature = "avx512f"),
            not(all(target_feature = "avx2", target_feature = "fma"))
        ))]
        {
            multicore::scalar_mt::axpy(alpha, x, y);
        }
        #[cfg(all(not(feature = "static-multicore"), target_feature = "avx512f"))]
        {
            avx512::axpy(alpha, x, y);
        }
        #[cfg(all(
            not(feature = "static-multicore"),
            not(target_feature = "avx512f"),
            target_feature = "avx2",
            target_feature = "fma"
        ))]
        {
            avx2::axpy(alpha, x, y);
        }
        #[cfg(all(
            not(feature = "static-multicore"),
            not(target_feature = "avx512f"),
            not(all(target_feature = "avx2", target_feature = "fma")),
            target_feature = "sse4.2"
        ))]
        {
            sse42::axpy(alpha, x, y);
        }
        #[cfg(all(
            not(feature = "static-multicore"),
            not(target_feature = "avx512f"),
            not(all(target_feature = "avx2", target_feature = "fma")),
            not(target_feature = "sse4.2")
        ))]
        {
            scalar::axpy(alpha, x, y);
        }
    }

    pub fn relu(x: &Tensor) -> Tensor {
        #[cfg(all(feature = "static-multicore", target_feature = "avx512f"))]
        {
            multicore::avx512_mt::relu(x)
        }
        #[cfg(all(
            feature = "static-multicore",
            not(target_feature = "avx512f"),
            target_feature = "avx2",
            target_feature = "fma"
        ))]
        {
            multicore::avx2_mt::relu(x)
        }
        #[cfg(all(
            feature = "static-multicore",
            not(target_feature = "avx512f"),
            not(all(target_feature = "avx2", target_feature = "fma"))
        ))]
        {
            multicore::scalar_mt::relu(x)
        }
        #[cfg(all(not(feature = "static-multicore"), target_feature = "avx512f"))]
        {
            avx512::relu(x)
        }
        #[cfg(all(
            not(feature = "static-multicore"),
            not(target_feature = "avx512f"),
            target_feature = "avx2",
            target_feature = "fma"
        ))]
        {
            avx2::relu(x)
        }
        #[cfg(all(
            not(feature = "static-multicore"),
            not(target_feature = "avx512f"),
            not(all(target_feature = "avx2", target_feature = "fma")),
            target_feature = "sse4.2"
        ))]
        {
            sse42::relu(x)
        }
        #[cfg(all(
            not(feature = "static-multicore"),
            not(target_feature = "avx512f"),
            not(all(target_feature = "avx2", target_feature = "fma")),
            not(target_feature = "sse4.2")
        ))]
        {
            scalar::relu(x)
        }
    }

    pub fn matvec(a: &Tensor, x: &Tensor) -> Tensor {
        #[cfg(all(feature = "static-multicore", target_feature = "avx512f"))]
        {
            multicore::avx512_mt::matvec(a, x)
        }
        #[cfg(all(
            feature = "static-multicore",
            not(target_feature = "avx512f"),
            target_feature = "avx2",
            target_feature = "fma"
        ))]
        {
            multicore::avx2_mt::matvec(a, x)
        }
        #[cfg(all(
            feature = "static-multicore",
            not(target_feature = "avx512f"),
            not(all(target_feature = "avx2", target_feature = "fma"))
        ))]
        {
            multicore::scalar_mt::matvec(a, x)
        }
        #[cfg(all(not(feature = "static-multicore"), target_feature = "avx512f"))]
        {
            avx512::matvec(a, x)
        }
        #[cfg(all(
            not(feature = "static-multicore"),
            not(target_feature = "avx512f"),
            target_feature = "avx2",
            target_feature = "fma"
        ))]
        {
            avx2::matvec(a, x)
        }
        #[cfg(all(
            not(feature = "static-multicore"),
            not(target_feature = "avx512f"),
            not(all(target_feature = "avx2", target_feature = "fma")),
            target_feature = "sse4.2"
        ))]
        {
            sse42::matvec(a, x)
        }
        #[cfg(all(
            not(feature = "static-multicore"),
            not(target_feature = "avx512f"),
            not(all(target_feature = "avx2", target_feature = "fma")),
            not(target_feature = "sse4.2")
        ))]
        {
            scalar::matvec(a, x)
        }
    }

    pub fn matmul(a: &Tensor, b: &Tensor) -> Tensor {
        #[cfg(all(feature = "static-multicore", target_feature = "avx512f"))]
        {
            multicore::avx512_mt::matmul(a, b)
        }
        #[cfg(all(
            feature = "static-multicore",
            not(target_feature = "avx512f"),
            target_feature = "avx2",
            target_feature = "fma"
        ))]
        {
            multicore::avx2_mt::matmul(a, b)
        }
        #[cfg(all(
            feature = "static-multicore",
            not(target_feature = "avx512f"),
            not(all(target_feature = "avx2", target_feature = "fma"))
        ))]
        {
            multicore::scalar_mt::matmul(a, b)
        }
        #[cfg(all(not(feature = "static-multicore"), target_feature = "avx512f"))]
        {
            avx512::matmul(a, b)
        }
        #[cfg(all(
            not(feature = "static-multicore"),
            not(target_feature = "avx512f"),
            target_feature = "avx2",
            target_feature = "fma"
        ))]
        {
            avx2::matmul(a, b)
        }
        #[cfg(all(
            not(feature = "static-multicore"),
            not(target_feature = "avx512f"),
            not(all(target_feature = "avx2", target_feature = "fma")),
            target_feature = "sse4.2"
        ))]
        {
            sse42::matmul(a, b)
        }
        #[cfg(all(
            not(feature = "static-multicore"),
            not(target_feature = "avx512f"),
            not(all(target_feature = "avx2", target_feature = "fma")),
            not(target_feature = "sse4.2")
        ))]
        {
            scalar::matmul(a, b)
        }
    }
}

/// Zero-dispatch direct baseline matching the selected backend name,
/// used to isolate pure `platform-aware` dispatch overhead.
#[inline(never)]
pub fn static_dispatch_probe(x: u64) -> u64 {
    x.wrapping_add(1)
}

pub fn direct_dot(backend: &str, a: &Tensor, b: &Tensor) -> f32 {
    match backend {
        "sse42" => sse42::dot(a, b),
        "avx2" => avx2::dot(a, b),
        "avx512" => avx512::dot(a, b),
        "multicore_scalar" => multicore::scalar_mt::dot(a, b),
        "multicore_avx2" => multicore::avx2_mt::dot(a, b),
        "multicore_avx512" => multicore::avx512_mt::dot(a, b),
        _ => scalar::dot(a, b),
    }
}

pub fn direct_axpy(backend: &str, alpha: f32, x: &Tensor, y: &mut Tensor) {
    match backend {
        "sse42" => sse42::axpy(alpha, x, y),
        "avx2" => avx2::axpy(alpha, x, y),
        "avx512" => avx512::axpy(alpha, x, y),
        "multicore_scalar" => multicore::scalar_mt::axpy(alpha, x, y),
        "multicore_avx2" => multicore::avx2_mt::axpy(alpha, x, y),
        "multicore_avx512" => multicore::avx512_mt::axpy(alpha, x, y),
        _ => scalar::axpy(alpha, x, y),
    }
}

pub fn direct_relu(backend: &str, x: &Tensor) -> Tensor {
    match backend {
        "sse42" => sse42::relu(x),
        "avx2" => avx2::relu(x),
        "avx512" => avx512::relu(x),
        "multicore_scalar" => multicore::scalar_mt::relu(x),
        "multicore_avx2" => multicore::avx2_mt::relu(x),
        "multicore_avx512" => multicore::avx512_mt::relu(x),
        _ => scalar::relu(x),
    }
}

pub fn direct_matvec(backend: &str, a: &Tensor, x: &Tensor) -> Tensor {
    match backend {
        "sse42" => sse42::matvec(a, x),
        "avx2" => avx2::matvec(a, x),
        "avx512" => avx512::matvec(a, x),
        "multicore_scalar" => multicore::scalar_mt::matvec(a, x),
        "multicore_avx2" => multicore::avx2_mt::matvec(a, x),
        "multicore_avx512" => multicore::avx512_mt::matvec(a, x),
        _ => scalar::matvec(a, x),
    }
}

pub fn direct_matmul(backend: &str, a: &Tensor, b: &Tensor) -> Tensor {
    match backend {
        "sse42" => sse42::matmul(a, b),
        "avx2" => avx2::matmul(a, b),
        "avx512" => avx512::matmul(a, b),
        "multicore_scalar" => multicore::scalar_mt::matmul(a, b),
        "multicore_avx2" => multicore::avx2_mt::matmul(a, b),
        "multicore_avx512" => multicore::avx512_mt::matmul(a, b),
        _ => scalar::matmul(a, b),
    }
}

/// Ad-hoc runtime feature detection (Sections 2.1 and 3.2):
/// platform-dispatch conditionals are embedded inside every operation.
pub struct AdHocRuntimeEngine;

impl AdHocRuntimeEngine {
    pub fn active_backend() -> &'static str {
        let multi = worker_count() >= 2;
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx512f") {
                return if multi { "multicore_avx512" } else { "avx512" };
            }
            if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
                return if multi { "multicore_avx2" } else { "avx2" };
            }
            if multi {
                return "multicore_scalar";
            }
            if is_x86_feature_detected!("sse4.2") {
                return "sse42";
            }
        }
        if multi {
            "multicore_scalar"
        } else {
            "scalar"
        }
    }

    #[inline(never)]
    pub fn dispatch_probe(x: u64) -> u64 {
        let multi = worker_count() >= 2;
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx512f") {
                return if multi {
                    x.wrapping_add(1)
                } else {
                    x.wrapping_add(1)
                };
            }
            if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
                return if multi {
                    x.wrapping_add(1)
                } else {
                    x.wrapping_add(1)
                };
            }
            if multi {
                return x.wrapping_add(1);
            }
            if is_x86_feature_detected!("sse4.2") {
                return x.wrapping_add(1);
            }
        }
        x.wrapping_add(1)
    }

    pub fn dot(a: &Tensor, b: &Tensor) -> f32 {
        let multi = worker_count() >= 2;
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx512f") {
                return if multi {
                    multicore::avx512_mt::dot(a, b)
                } else {
                    avx512::dot(a, b)
                };
            }
            if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
                return if multi {
                    multicore::avx2_mt::dot(a, b)
                } else {
                    avx2::dot(a, b)
                };
            }
            if multi {
                return multicore::scalar_mt::dot(a, b);
            }
            if is_x86_feature_detected!("sse4.2") {
                return sse42::dot(a, b);
            }
        }
        if multi {
            multicore::scalar_mt::dot(a, b)
        } else {
            scalar::dot(a, b)
        }
    }

    pub fn axpy(alpha: f32, x: &Tensor, y: &mut Tensor) {
        let multi = worker_count() >= 2;
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx512f") {
                if multi {
                    multicore::avx512_mt::axpy(alpha, x, y);
                } else {
                    avx512::axpy(alpha, x, y);
                }
                return;
            }
            if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
                if multi {
                    multicore::avx2_mt::axpy(alpha, x, y);
                } else {
                    avx2::axpy(alpha, x, y);
                }
                return;
            }
            if multi {
                multicore::scalar_mt::axpy(alpha, x, y);
                return;
            }
            if is_x86_feature_detected!("sse4.2") {
                sse42::axpy(alpha, x, y);
                return;
            }
        }
        if multi {
            multicore::scalar_mt::axpy(alpha, x, y);
        } else {
            scalar::axpy(alpha, x, y);
        }
    }

    pub fn relu(x: &Tensor) -> Tensor {
        let multi = worker_count() >= 2;
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx512f") {
                return if multi {
                    multicore::avx512_mt::relu(x)
                } else {
                    avx512::relu(x)
                };
            }
            if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
                return if multi {
                    multicore::avx2_mt::relu(x)
                } else {
                    avx2::relu(x)
                };
            }
            if multi {
                return multicore::scalar_mt::relu(x);
            }
            if is_x86_feature_detected!("sse4.2") {
                return sse42::relu(x);
            }
        }
        if multi {
            multicore::scalar_mt::relu(x)
        } else {
            scalar::relu(x)
        }
    }

    pub fn matvec(a: &Tensor, x: &Tensor) -> Tensor {
        let multi = worker_count() >= 2;
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx512f") {
                return if multi {
                    multicore::avx512_mt::matvec(a, x)
                } else {
                    avx512::matvec(a, x)
                };
            }
            if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
                return if multi {
                    multicore::avx2_mt::matvec(a, x)
                } else {
                    avx2::matvec(a, x)
                };
            }
            if multi {
                return multicore::scalar_mt::matvec(a, x);
            }
            if is_x86_feature_detected!("sse4.2") {
                return sse42::matvec(a, x);
            }
        }
        if multi {
            multicore::scalar_mt::matvec(a, x)
        } else {
            scalar::matvec(a, x)
        }
    }

    pub fn matmul(a: &Tensor, b: &Tensor) -> Tensor {
        let multi = worker_count() >= 2;
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx512f") {
                return if multi {
                    multicore::avx512_mt::matmul(a, b)
                } else {
                    avx512::matmul(a, b)
                };
            }
            if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
                return if multi {
                    multicore::avx2_mt::matmul(a, b)
                } else {
                    avx2::matmul(a, b)
                };
            }
            if multi {
                return multicore::scalar_mt::matmul(a, b);
            }
            if is_x86_feature_detected!("sse4.2") {
                return sse42::matmul(a, b);
            }
        }
        if multi {
            multicore::scalar_mt::matmul(a, b)
        } else {
            scalar::matmul(a, b)
        }
    }
}
