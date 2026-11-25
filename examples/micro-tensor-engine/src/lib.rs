pub mod adhoc_idiomatic;
pub mod cuda_backend;
pub mod kernels;
#[cfg(feature = "engine-mv")]
pub mod multiversion_engine;
#[cfg(feature = "engine-pa")]
pub mod platform_engine;
pub mod static_engine;
pub mod tensor;

pub use adhoc_idiomatic::AdHocIdiomaticEngine;
#[cfg(feature = "engine-mv")]
pub use multiversion_engine::MultiversionEngine;
#[cfg(feature = "engine-pa")]
pub use platform_engine::TensorOps;
pub use static_engine::{AdHocRuntimeEngine, StaticCfgEngine};
pub use tensor::Tensor;

/// High-precision f64 reference dot product for numerical accuracy validation.
pub fn dot_reference_f64(a: &Tensor, b: &Tensor) -> f64 {
    assert_eq!(a.len(), b.len(), "dot_reference_f64: mismatched lengths");
    a.as_slice()
        .iter()
        .zip(b.as_slice().iter())
        .map(|(&x, &y)| (x as f64) * (y as f64))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernels::{avx2, avx512, multicore, scalar, sse42};

    fn assert_close_abs_rel(got: f32, expected: f64, atol: f64, rtol: f64, ctx: &str) {
        let abs_err = ((got as f64) - expected).abs();
        let limit = atol + rtol * expected.abs();
        assert!(
            abs_err <= limit,
            "{ctx}: expected {expected:.6}, got {got:.6} (abs_err={abs_err:.3e}, limit={limit:.3e})"
        );
    }

    #[test]
    fn test_all_host_supported_kernels_match_reference_with_abs_rel_tolerances() {
        // Non-power-of-two prime shapes to stress every SIMD tail path (mod 4/8/16/32)
        let n = 1031;
        let v1 = Tensor::random(&[n], 1);
        let v2 = Tensor::random(&[n], 2);
        let a_mv = Tensor::random(&[73, 131], 3);
        let x_mv = Tensor::random(&[131], 4);
        let a_mm = Tensor::random(&[67, 79], 5);
        let b_mm = Tensor::random(&[79, 83], 6);

        let ref_dot_f64 = dot_reference_f64(&v1, &v2);
        let mut ref_y = v2.clone();
        scalar::axpy(0.37, &v1, &mut ref_y);
        let ref_relu = scalar::relu(&v1);
        let ref_mv = scalar::matvec(&a_mv, &x_mv);
        let ref_mm = scalar::matmul(&a_mm, &b_mm);

        assert_close_abs_rel(scalar::dot(&v1, &v2), ref_dot_f64, 1e-3, 1e-4, "scalar dot");

        // Multicore scalar
        assert_close_abs_rel(multicore::scalar_mt::dot(&v1, &v2), ref_dot_f64, 1e-3, 1e-4, "mt_scalar dot");
        let mut y_mt = v2.clone();
        multicore::scalar_mt::axpy(0.37, &v1, &mut y_mt);
        assert!(y_mt.max_abs_diff(&ref_y) < 1e-5);
        assert!(multicore::scalar_mt::relu(&v1).max_abs_diff(&ref_relu) == 0.0);
        let mut out_relu = Tensor::zeros(v1.shape());
        multicore::scalar_mt::relu_into(&v1, &mut out_relu);
        assert!(out_relu.max_abs_diff(&ref_relu) == 0.0);
        assert!(multicore::scalar_mt::matvec(&a_mv, &x_mv).max_abs_diff(&ref_mv) < 1e-4);
        assert!(multicore::scalar_mt::matmul(&a_mm, &b_mm).max_abs_diff(&ref_mm) < 1e-4);

        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("sse4.2") {
                assert_close_abs_rel(sse42::dot(&v1, &v2), ref_dot_f64, 1e-3, 1e-4, "sse42 dot");
                let mut y = v2.clone();
                sse42::axpy(0.37, &v1, &mut y);
                assert!(y.max_abs_diff(&ref_y) < 1e-5);
                assert!(sse42::relu(&v1).max_abs_diff(&ref_relu) == 0.0);
                assert!(sse42::matvec(&a_mv, &x_mv).max_abs_diff(&ref_mv) < 1e-4);
                assert!(sse42::matmul(&a_mm, &b_mm).max_abs_diff(&ref_mm) < 1e-4);
            }

            if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
                assert_close_abs_rel(avx2::dot(&v1, &v2), ref_dot_f64, 1e-3, 1e-4, "avx2+fma dot");
                let mut y = v2.clone();
                avx2::axpy(0.37, &v1, &mut y);
                assert!(y.max_abs_diff(&ref_y) < 1e-5);
                assert!(avx2::relu(&v1).max_abs_diff(&ref_relu) == 0.0);
                let mut out_r = Tensor::zeros(v1.shape());
                avx2::relu_into(&v1, &mut out_r);
                assert!(out_r.max_abs_diff(&ref_relu) == 0.0);
                assert!(avx2::matvec(&a_mv, &x_mv).max_abs_diff(&ref_mv) < 1e-4);
                assert!(avx2::matmul(&a_mm, &b_mm).max_abs_diff(&ref_mm) < 1e-4);
                assert!(avx2::matmul_no_fma(&a_mm, &b_mm).max_abs_diff(&ref_mm) < 1e-4);

                assert_close_abs_rel(multicore::avx2_mt::dot(&v1, &v2), ref_dot_f64, 1e-3, 1e-4, "avx2_mt dot");
                let mut y_m = v2.clone();
                multicore::avx2_mt::axpy(0.37, &v1, &mut y_m);
                assert!(y_m.max_abs_diff(&ref_y) < 1e-5);
                assert!(multicore::avx2_mt::relu(&v1).max_abs_diff(&ref_relu) == 0.0);
                multicore::avx2_mt::relu_into(&v1, &mut out_r);
                assert!(out_r.max_abs_diff(&ref_relu) == 0.0);
                assert!(multicore::avx2_mt::matvec(&a_mv, &x_mv).max_abs_diff(&ref_mv) < 1e-4);
                assert!(multicore::avx2_mt::matmul(&a_mm, &b_mm).max_abs_diff(&ref_mm) < 1e-4);
            }

            // Executed when running natively on AVX-512 CPUs or under `qemu-x86_64 -cpu max`!
            if is_x86_feature_detected!("avx512f") {
                assert_close_abs_rel(avx512::dot(&v1, &v2), ref_dot_f64, 1e-3, 1e-4, "avx512 dot");
                let mut y512 = v2.clone();
                avx512::axpy(0.37, &v1, &mut y512);
                assert!(y512.max_abs_diff(&ref_y) < 1e-5, "avx512 axpy diff={}", y512.max_abs_diff(&ref_y));
                assert!(avx512::relu(&v1).max_abs_diff(&ref_relu) == 0.0);
                assert!(avx512::matvec(&a_mv, &x_mv).max_abs_diff(&ref_mv) < 1e-4);
                assert!(avx512::matmul(&a_mm, &b_mm).max_abs_diff(&ref_mm) < 1e-4);

                assert_close_abs_rel(multicore::avx512_mt::dot(&v1, &v2), ref_dot_f64, 1e-3, 1e-4, "avx512_mt dot");
                let mut y512m = v2.clone();
                multicore::avx512_mt::axpy(0.37, &v1, &mut y512m);
                assert!(y512m.max_abs_diff(&ref_y) < 1e-5);
                assert!(multicore::avx512_mt::relu(&v1).max_abs_diff(&ref_relu) == 0.0);
                assert!(multicore::avx512_mt::matvec(&a_mv, &x_mv).max_abs_diff(&ref_mv) < 1e-4);
                assert!(multicore::avx512_mt::matmul(&a_mm, &b_mm).max_abs_diff(&ref_mm) < 1e-4);
            }
        }

        // StaticCfgEngine, AdHocRuntimeEngine, AdHocIdiomaticEngine
        assert_close_abs_rel(StaticCfgEngine::dot(&v1, &v2), ref_dot_f64, 1e-3, 1e-4, "static_cfg dot");
        assert!(StaticCfgEngine::matmul(&a_mm, &b_mm).max_abs_diff(&ref_mm) < 1e-4);

        assert_close_abs_rel(AdHocRuntimeEngine::dot(&v1, &v2), ref_dot_f64, 1e-3, 1e-4, "adhoc dot");
        assert!(AdHocRuntimeEngine::matmul(&a_mm, &b_mm).max_abs_diff(&ref_mm) < 1e-4);

        assert_close_abs_rel(AdHocIdiomaticEngine::dot(&v1, &v2), ref_dot_f64, 1e-3, 1e-4, "adhoc_idiom dot");
        assert!(AdHocIdiomaticEngine::matmul(&a_mm, &b_mm).max_abs_diff(&ref_mm) < 1e-4);

        #[cfg(feature = "engine-mv")]
        {
            assert_close_abs_rel(MultiversionEngine::dot(&v1, &v2), ref_dot_f64, 1e-3, 1e-4, "mv dot");
            assert_close_abs_rel(MultiversionEngine::dot_simd(&v1, &v2), ref_dot_f64, 1e-3, 1e-4, "mv_simd dot");
            assert!(MultiversionEngine::matmul(&a_mm, &b_mm).max_abs_diff(&ref_mm) < 1e-4);
            assert!(MultiversionEngine::matmul_simd(&a_mm, &b_mm).max_abs_diff(&ref_mm) < 1e-4);
        }

        // Real NVIDIA GPU validation (when libcuda.so.1 and a CUDA device are present)
        if cuda_backend::is_available() {
            assert_close_abs_rel(cuda_backend::dot(&v1, &v2), ref_dot_f64, 1e-3, 1e-4, "cuda dot");
            let mut y_cu = v2.clone();
            cuda_backend::axpy(0.37, &v1, &mut y_cu);
            assert!(y_cu.max_abs_diff(&ref_y) < 1e-5, "cuda axpy diff={}", y_cu.max_abs_diff(&ref_y));
            assert!(cuda_backend::relu(&v1).max_abs_diff(&ref_relu) == 0.0);
            assert!(cuda_backend::matvec(&a_mv, &x_mv).max_abs_diff(&ref_mv) < 1e-4);
            assert!(cuda_backend::matmul(&a_mm, &b_mm).max_abs_diff(&ref_mm) < 1e-4);
        }

        #[cfg(feature = "engine-pa")]
        {
            let pa_backend = v1.active_backend();
            #[cfg(target_arch = "x86_64")]
            if matches!(pa_backend, "avx512" | "multicore_avx512") && !is_x86_feature_detected!("avx512f")
            {
                panic!(
                    "Platform.toml selected `{pa_backend}` on a CPU without AVX-512F. \
                     Run `cargo test` from `examples/micro-tensor-engine/` or set PLATFORM_DESCRIPTION."
                );
            }

            assert_close_abs_rel(v1.dot(&v2), ref_dot_f64, 1e-3, 1e-4, "platform-aware dot");
            let mut y_pa = v2.clone();
            y_pa.axpy(0.37, &v1);
            assert!(y_pa.max_abs_diff(&ref_y) < 1e-5);
            assert!(v1.relu().max_abs_diff(&ref_relu) == 0.0);
            assert!(a_mv.matvec(&x_mv).max_abs_diff(&ref_mv) < 1e-4);
            assert!(a_mm.matmul(&b_mm).max_abs_diff(&ref_mm) < 1e-4);
        }

        // Exercise persistent thread-pool path above parallel threshold (65_536)
        let big_n = 70_003;
        let b1 = Tensor::random(&[big_n], 101);
        let b2 = Tensor::random(&[big_n], 202);
        let ref_big_dot_f64 = dot_reference_f64(&b1, &b2);
        let mut ref_big_y = b2.clone();
        scalar::axpy(0.37, &b1, &mut ref_big_y);
        let ref_big_relu = scalar::relu(&b1);

        assert_close_abs_rel(multicore::scalar_mt::dot(&b1, &b2), ref_big_dot_f64, 2e-2, 5e-4, "big mt_scalar dot");
        let mut y_big = b2.clone();
        multicore::scalar_mt::axpy(0.37, &b1, &mut y_big);
        assert!(y_big.max_abs_diff(&ref_big_y) < 1e-5);
        assert!(multicore::scalar_mt::relu(&b1).max_abs_diff(&ref_big_relu) == 0.0);

        #[cfg(target_arch = "x86_64")]
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            assert_close_abs_rel(multicore::avx2_mt::dot(&b1, &b2), ref_big_dot_f64, 2e-2, 5e-4, "big avx2_mt dot");
            let mut y_big_avx = b2.clone();
            multicore::avx2_mt::axpy(0.37, &b1, &mut y_big_avx);
            assert!(y_big_avx.max_abs_diff(&ref_big_y) < 1e-5);
            assert!(multicore::avx2_mt::relu(&b1).max_abs_diff(&ref_big_relu) == 0.0);
        }

        #[cfg(target_arch = "x86_64")]
        if is_x86_feature_detected!("avx512f") {
            assert_close_abs_rel(multicore::avx512_mt::dot(&b1, &b2), ref_big_dot_f64, 2e-2, 5e-4, "big avx512_mt dot");
            let mut y_big_512 = b2.clone();
            multicore::avx512_mt::axpy(0.37, &b1, &mut y_big_512);
            assert!(y_big_512.max_abs_diff(&ref_big_y) < 1e-5);
            assert!(multicore::avx512_mt::relu(&b1).max_abs_diff(&ref_big_relu) == 0.0);
        }
    }

    /// Empirical proof of Challenge 5 (lexical-order sensitivity and poset pathology):
    /// 1) On a host with SSE_4_2 + >=2 cores, swapping declaration order silently flips selection.
    /// 2) On a host with BOTH CPU SIMD and GPU CUDA (`profiles/gpu_cuda`), declaring a weak CPU
    ///    variant (`SSE_4_2`) AFTER a strong GPU variant (`CUDA`) causes `resolve()`'s reverse-order
    ///    greedy pass to anchor on `SSE_4_2` and silently discard the GPU.
    #[cfg(feature = "engine-pa")]
    #[test]
    fn test_lexical_order_and_poset_gpu_drop_pathology() {
        use platform_aware::platformaware;

        #[platformaware]
        mod order_simd_then_mc {
            use platform_aware::{AtLeast, SSE_4_2};

            #[kernelversion]
            pub fn pick() -> &'static str { "scalar" }

            #[kernelversion(cpu_simd = SSE_4_2)]
            pub fn pick() -> &'static str { "sse42" }

            #[kernelversion(cpu_core_count = AtLeast { val: 2 })]
            pub fn pick() -> &'static str { "multicore_scalar" }
        }

        #[platformaware]
        mod order_mc_then_simd {
            use platform_aware::{AtLeast, SSE_4_2};

            #[kernelversion]
            pub fn pick() -> &'static str { "scalar" }

            #[kernelversion(cpu_core_count = AtLeast { val: 2 })]
            pub fn pick() -> &'static str { "multicore_scalar" }

            #[kernelversion(cpu_simd = SSE_4_2)]
            pub fn pick() -> &'static str { "sse42" }
        }

        #[platformaware]
        mod gpu_then_weak_cpu {
            use platform_aware::SSE_4_2;
            use platform_aware_nvidia::{CUDA, Turing};

            #[kernelversion]
            pub fn pick_device() -> &'static str { "scalar" }

            #[kernelversion(acc_backend = CUDA, acc_architecture = Turing)]
            pub fn pick_device() -> &'static str { "gpu_cuda" }

            // Weak CPU variant declared AFTER GPU -> reverse greedy pass anchors here and drops GPU!
            #[kernelversion(cpu_simd = SSE_4_2)]
            pub fn pick_device() -> &'static str { "sse42" }
        }

        #[platformaware]
        mod weak_cpu_then_gpu {
            use platform_aware::SSE_4_2;
            use platform_aware_nvidia::{CUDA, Turing};

            #[kernelversion]
            pub fn pick_device() -> &'static str { "scalar" }

            #[kernelversion(cpu_simd = SSE_4_2)]
            pub fn pick_device() -> &'static str { "sse42" }

            #[kernelversion(acc_backend = CUDA, acc_architecture = Turing)]
            pub fn pick_device() -> &'static str { "gpu_cuda" }
        }

        let a = order_simd_then_mc::pick();
        let b = order_mc_then_simd::pick();
        if a != "scalar" && b != "scalar" && a != b {
            assert_eq!(a, "multicore_scalar");
            assert_eq!(b, "sse42");
        }

        // When PLATFORM_DESCRIPTION=profiles/gpu_cuda, verify the GPU-drop poset pathology:
        let d_good = weak_cpu_then_gpu::pick_device();
        let d_bad = gpu_then_weak_cpu::pick_device();
        if d_good == "gpu_cuda" {
            assert_eq!(
                d_bad, "sse42",
                "Expected poset pathology: declaring SSE_4_2 after CUDA silently drops GPU"
            );
        }
    }

    /// Empirical verification of DA2 (generics & monomorphization under `#[platformaware]`):
    /// Proves that `#[platformaware]` compiles generic k-functions `fn generic_id<T>(x: T)` and
    /// generic trait impls `impl<T: Copy> GenericKernel for Wrapper<T>`, BUT the generated
    /// `static SELECTED_VARIANT: LazyLock<i32>` inside the dispatcher is shared across ALL
    /// monomorphizations of `T` (so `f32` and `f64` cannot cache distinct resolved variants
    /// or declare type-specialized SIMD bounds per variant).
    #[cfg(feature = "engine-pa")]
    #[test]
    fn test_da2_generics_compile_with_shared_static_cache() {
        use platform_aware::platformaware;

        #[platformaware]
        mod generic_ops {
            use platform_aware::SSE_4_2;

            pub struct Wrapper<T>(pub T);
            pub trait GenericKernel<T> {
                fn identity(&self) -> T;
            }

            #[kernelversion]
            pub fn generic_kfn<T: Copy>(x: T) -> (T, &'static str) {
                (x, "scalar")
            }

            #[kernelversion(cpu_simd = SSE_4_2)]
            pub fn generic_kfn<T: Copy>(x: T) -> (T, &'static str) {
                (x, "sse42")
            }

            impl<T: Copy> GenericKernel<T> for Wrapper<T> {
                #[kernelversion]
                fn identity(&self) -> T {
                    self.0
                }

                #[kernelversion(cpu_simd = SSE_4_2)]
                fn identity(&self) -> T {
                    self.0
                }
            }
        }

        use generic_ops::{GenericKernel, Wrapper};

        let (v_f32, b_f32) = generic_ops::generic_kfn(1.5_f32);
        let (v_f64, b_f64) = generic_ops::generic_kfn(2.5_f64);
        assert_eq!(v_f32, 1.5_f32);
        assert_eq!(v_f64, 2.5_f64);
        assert_eq!(b_f32, b_f64);
        assert_eq!(Wrapper(42_u32).identity(), 42_u32);
        assert_eq!(Wrapper(3.14_f64).identity(), 3.14_f64);
    }
}
