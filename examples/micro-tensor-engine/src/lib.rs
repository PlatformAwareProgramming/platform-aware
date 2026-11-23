pub mod kernels;
pub mod platform_engine;
pub mod static_engine;
pub mod tensor;

pub use platform_engine::TensorOps;
pub use static_engine::{AdHocRuntimeEngine, StaticCfgEngine};
pub use tensor::Tensor;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernels::{avx2, multicore, scalar, sse42};

    fn assert_close(a: f32, b: f32, tol: f32, ctx: &str) {
        assert!(
            (a - b).abs() <= tol,
            "{ctx}: expected {b}, got {a} (diff = {})",
            (a - b).abs()
        );
    }

    #[test]
    fn test_all_host_supported_kernels_match_scalar_reference() {
        // Use non-power-of-two shapes to stress every SIMD tail path (mod 4/8/16/32)
        let n = 1031;
        let v1 = Tensor::random(&[n], 1);
        let v2 = Tensor::random(&[n], 2);
        let a_mv = Tensor::random(&[73, 131], 3);
        let x_mv = Tensor::random(&[131], 4);
        let a_mm = Tensor::random(&[67, 79], 5);
        let b_mm = Tensor::random(&[79, 83], 6);

        let ref_dot = scalar::dot(&v1, &v2);
        let mut ref_y = v2.clone();
        scalar::axpy(0.37, &v1, &mut ref_y);
        let ref_relu = scalar::relu(&v1);
        let ref_mv = scalar::matvec(&a_mv, &x_mv);
        let ref_mm = scalar::matmul(&a_mm, &b_mm);

        // Multicore scalar
        assert_close(multicore::scalar_mt::dot(&v1, &v2), ref_dot, 1e-3, "mt_scalar dot");
        let mut y_mt = v2.clone();
        multicore::scalar_mt::axpy(0.37, &v1, &mut y_mt);
        assert!(y_mt.max_abs_diff(&ref_y) < 1e-5);
        assert!(multicore::scalar_mt::relu(&v1).max_abs_diff(&ref_relu) == 0.0);
        assert!(multicore::scalar_mt::matvec(&a_mv, &x_mv).max_abs_diff(&ref_mv) < 1e-4);
        assert!(multicore::scalar_mt::matmul(&a_mm, &b_mm).max_abs_diff(&ref_mm) < 1e-4);

        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("sse4.2") {
                assert_close(sse42::dot(&v1, &v2), ref_dot, 1e-3, "sse42 dot");
                let mut y = v2.clone();
                sse42::axpy(0.37, &v1, &mut y);
                assert!(y.max_abs_diff(&ref_y) < 1e-5);
                assert!(sse42::relu(&v1).max_abs_diff(&ref_relu) == 0.0);
                assert!(sse42::matvec(&a_mv, &x_mv).max_abs_diff(&ref_mv) < 1e-4);
                assert!(sse42::matmul(&a_mm, &b_mm).max_abs_diff(&ref_mm) < 1e-4);
            }

            if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
                assert_close(avx2::dot(&v1, &v2), ref_dot, 1e-3, "avx2 dot");
                let mut y = v2.clone();
                avx2::axpy(0.37, &v1, &mut y);
                assert!(y.max_abs_diff(&ref_y) < 1e-5);
                assert!(avx2::relu(&v1).max_abs_diff(&ref_relu) == 0.0);
                assert!(avx2::matvec(&a_mv, &x_mv).max_abs_diff(&ref_mv) < 1e-4);
                assert!(avx2::matmul(&a_mm, &b_mm).max_abs_diff(&ref_mm) < 1e-4);

                assert_close(multicore::avx2_mt::dot(&v1, &v2), ref_dot, 1e-3, "avx2_mt dot");
                let mut y_m = v2.clone();
                multicore::avx2_mt::axpy(0.37, &v1, &mut y_m);
                assert!(y_m.max_abs_diff(&ref_y) < 1e-5);
                assert!(multicore::avx2_mt::relu(&v1).max_abs_diff(&ref_relu) == 0.0);
                assert!(multicore::avx2_mt::matvec(&a_mv, &x_mv).max_abs_diff(&ref_mv) < 1e-4);
                assert!(multicore::avx2_mt::matmul(&a_mm, &b_mm).max_abs_diff(&ref_mm) < 1e-4);
            }
        }

        // StaticCfgEngine & AdHocRuntimeEngine & Platform-Aware TensorOps
        assert_close(StaticCfgEngine::dot(&v1, &v2), ref_dot, 1e-3, "static_cfg dot");
        assert!(StaticCfgEngine::matmul(&a_mm, &b_mm).max_abs_diff(&ref_mm) < 1e-4);

        assert_close(AdHocRuntimeEngine::dot(&v1, &v2), ref_dot, 1e-3, "adhoc dot");
        assert!(AdHocRuntimeEngine::matmul(&a_mm, &b_mm).max_abs_diff(&ref_mm) < 1e-4);

        let pa_backend = v1.active_backend();
        #[cfg(target_arch = "x86_64")]
        if matches!(pa_backend, "avx512" | "multicore_avx512") && !is_x86_feature_detected!("avx512f")
        {
            panic!(
                "Platform.toml selected `{pa_backend}` on a CPU without AVX-512F. \
                 Run `cargo test` from `examples/micro-tensor-engine/` or set PLATFORM_DESCRIPTION."
            );
        }

        assert_close(v1.dot(&v2), ref_dot, 1e-3, "platform-aware dot");
        let mut y_pa = v2.clone();
        y_pa.axpy(0.37, &v1);
        assert!(y_pa.max_abs_diff(&ref_y) < 1e-5);
        assert!(v1.relu().max_abs_diff(&ref_relu) == 0.0);
        assert!(a_mv.matvec(&x_mv).max_abs_diff(&ref_mv) < 1e-4);
        assert!(a_mm.matmul(&b_mm).max_abs_diff(&ref_mm) < 1e-4);

        // Also exercise the persistent thread-pool path above the parallel threshold (65_536)
        let big_n = 70_003;
        let b1 = Tensor::random(&[big_n], 101);
        let b2 = Tensor::random(&[big_n], 202);
        let ref_big_dot = scalar::dot(&b1, &b2);
        let mut ref_big_y = b2.clone();
        scalar::axpy(0.37, &b1, &mut ref_big_y);
        let ref_big_relu = scalar::relu(&b1);

        assert_close(multicore::scalar_mt::dot(&b1, &b2), ref_big_dot, 2e-2, "big mt_scalar dot");
        let mut y_big = b2.clone();
        multicore::scalar_mt::axpy(0.37, &b1, &mut y_big);
        assert!(y_big.max_abs_diff(&ref_big_y) < 1e-5);
        assert!(multicore::scalar_mt::relu(&b1).max_abs_diff(&ref_big_relu) == 0.0);

        #[cfg(target_arch = "x86_64")]
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            assert_close(multicore::avx2_mt::dot(&b1, &b2), ref_big_dot, 2e-2, "big avx2_mt dot");
            let mut y_big_avx = b2.clone();
            multicore::avx2_mt::axpy(0.37, &b1, &mut y_big_avx);
            assert!(y_big_avx.max_abs_diff(&ref_big_y) < 1e-5);
            assert!(multicore::avx2_mt::relu(&b1).max_abs_diff(&ref_big_relu) == 0.0);
        }
    }
}
