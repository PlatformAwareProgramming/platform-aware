//! Shared low-level tensor kernels (Scalar, SSE4.2, AVX2+FMA, AVX-512, and Multicore).
//!
//! Both the `platform-aware` engine and the static/ad-hoc baselines call these exact
//! kernels so benchmarks measure true dispatch overhead and performance-portability
//! differences without implementation skew.

use std::sync::OnceLock;

use crate::tensor::Tensor;

pub fn worker_count() -> usize {
    static WORKERS: OnceLock<usize> = OnceLock::new();
    *WORKERS.get_or_init(|| {
        std::env::var("TENSOR_THREADS")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .filter(|&n| n >= 1)
            .unwrap_or_else(|| {
                std::thread::available_parallelism()
                    .map(|n| n.get())
                    .unwrap_or(4)
            })
    })
}

/// Scalar portable reference implementations (fallback).
pub mod scalar {
    use super::Tensor;

    #[inline(never)]
    pub fn dot(a: &Tensor, b: &Tensor) -> f32 {
        assert_eq!(a.len(), b.len(), "dot: mismatched lengths");
        let (xs, ys) = (a.as_slice(), b.as_slice());
        let mut s0 = 0.0_f32;
        let mut s1 = 0.0_f32;
        let mut s2 = 0.0_f32;
        let mut s3 = 0.0_f32;
        let mut i = 0;
        let n = xs.len();
        while i + 4 <= n {
            s0 += xs[i] * ys[i];
            s1 += xs[i + 1] * ys[i + 1];
            s2 += xs[i + 2] * ys[i + 2];
            s3 += xs[i + 3] * ys[i + 3];
            i += 4;
        }
        let mut acc = (s0 + s1) + (s2 + s3);
        while i < n {
            acc += xs[i] * ys[i];
            i += 1;
        }
        acc
    }

    #[inline(never)]
    pub fn axpy(alpha: f32, x: &Tensor, y: &mut Tensor) {
        assert_eq!(x.len(), y.len(), "axpy: mismatched lengths");
        let xs = x.as_slice();
        let ys = y.as_mut_slice();
        for i in 0..xs.len() {
            ys[i] += alpha * xs[i];
        }
    }

    #[inline(never)]
    pub fn relu(x: &Tensor) -> Tensor {
        let mut out = Tensor::zeros(x.shape());
        let xs = x.as_slice();
        let os = out.as_mut_slice();
        for i in 0..xs.len() {
            let v = xs[i];
            os[i] = if v > 0.0 { v } else { 0.0 };
        }
        out
    }

    #[inline(never)]
    pub fn matvec(a: &Tensor, x: &Tensor) -> Tensor {
        let (m, k) = a.dims_2d();
        assert_eq!(x.len(), k, "matvec: vector length {} != cols {}", x.len(), k);
        let mut y = Tensor::zeros(&[m]);
        matvec_rows(a.as_slice(), x.as_slice(), y.as_mut_slice(), k);
        y
    }

    pub(crate) fn matvec_rows(a_rows: &[f32], x: &[f32], y_rows: &mut [f32], k: usize) {
        for (row_idx, y_out) in y_rows.iter_mut().enumerate() {
            let row = &a_rows[row_idx * k..(row_idx + 1) * k];
            let mut sum = 0.0_f32;
            for j in 0..k {
                sum += row[j] * x[j];
            }
            *y_out = sum;
        }
    }

    #[inline(never)]
    pub fn matmul(a: &Tensor, b: &Tensor) -> Tensor {
        let (m, k) = a.dims_2d();
        let (bk, n) = b.dims_2d();
        assert_eq!(k, bk, "matmul: inner dims {} != {}", k, bk);
        let mut c = Tensor::zeros(&[m, n]);
        matmul_rows(a.as_slice(), b.as_slice(), c.as_mut_slice(), k, n);
        c
    }

    /// Cache-friendly `i-k-j` scalar GEMM over a contiguous slice of output rows.
    pub(crate) fn matmul_rows(a_rows: &[f32], b: &[f32], c_rows: &mut [f32], k: usize, n: usize) {
        let m_rows = c_rows.len() / n;
        for i in 0..m_rows {
            let c_row = &mut c_rows[i * n..(i + 1) * n];
            let a_row = &a_rows[i * k..(i + 1) * k];
            for p in 0..k {
                let a_ip = a_row[p];
                let b_row = &b[p * n..(p + 1) * n];
                for j in 0..n {
                    c_row[j] += a_ip * b_row[j];
                }
            }
        }
    }
}

/// Explicit x86_64 SSE4.2 kernels (128-bit SIMD, 4 x f32 per register).
pub mod sse42 {
    use super::Tensor;

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse4.2")]
    unsafe fn dot_impl(xs: &[f32], ys: &[f32]) -> f32 {
        use std::arch::x86_64::*;
        let n = xs.len();
        let mut i = 0;
        let mut acc0 = _mm_setzero_ps();
        let mut acc1 = _mm_setzero_ps();
        while i + 8 <= n {
            let a0 = _mm_loadu_ps(xs.as_ptr().add(i));
            let b0 = _mm_loadu_ps(ys.as_ptr().add(i));
            let a1 = _mm_loadu_ps(xs.as_ptr().add(i + 4));
            let b1 = _mm_loadu_ps(ys.as_ptr().add(i + 4));
            acc0 = _mm_add_ps(acc0, _mm_mul_ps(a0, b0));
            acc1 = _mm_add_ps(acc1, _mm_mul_ps(a1, b1));
            i += 8;
        }
        let mut acc = _mm_add_ps(acc0, acc1);
        while i + 4 <= n {
            let a = _mm_loadu_ps(xs.as_ptr().add(i));
            let b = _mm_loadu_ps(ys.as_ptr().add(i));
            acc = _mm_add_ps(acc, _mm_mul_ps(a, b));
            i += 4;
        }
        let mut tmp = [0.0_f32; 4];
        _mm_storeu_ps(tmp.as_mut_ptr(), acc);
        let mut sum = (tmp[0] + tmp[1]) + (tmp[2] + tmp[3]);
        while i < n {
            sum += xs[i] * ys[i];
            i += 1;
        }
        sum
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse4.2")]
    unsafe fn axpy_impl(alpha: f32, xs: &[f32], ys: &mut [f32]) {
        use std::arch::x86_64::*;
        let n = xs.len();
        let va = _mm_set1_ps(alpha);
        let mut i = 0;
        while i + 4 <= n {
            let vx = _mm_loadu_ps(xs.as_ptr().add(i));
            let vy = _mm_loadu_ps(ys.as_ptr().add(i));
            let res = _mm_add_ps(vy, _mm_mul_ps(va, vx));
            _mm_storeu_ps(ys.as_mut_ptr().add(i), res);
            i += 4;
        }
        while i < n {
            ys[i] += alpha * xs[i];
            i += 1;
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse4.2")]
    unsafe fn relu_impl(xs: &[f32], os: &mut [f32]) {
        use std::arch::x86_64::*;
        let n = xs.len();
        let zero = _mm_setzero_ps();
        let mut i = 0;
        while i + 4 <= n {
            let vx = _mm_loadu_ps(xs.as_ptr().add(i));
            _mm_storeu_ps(os.as_mut_ptr().add(i), _mm_max_ps(vx, zero));
            i += 4;
        }
        while i < n {
            os[i] = xs[i].max(0.0);
            i += 1;
        }
    }

    /// Cache-blocked 4x8 register micro-kernel GEMM with SSE4.2.
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse4.2")]
    unsafe fn matmul_rows_impl(a_rows: &[f32], b: &[f32], c_rows: &mut [f32], k: usize, n: usize) {
        use std::arch::x86_64::*;
        const BK: usize = 64;
        const BJ: usize = 128;

        let m_rows = c_rows.len() / n;
        let a_base = a_rows.as_ptr();
        let b_base = b.as_ptr();
        let c_base = c_rows.as_mut_ptr();

        let mut i = 0;
        while i + 4 <= m_rows {
            let mut p0 = 0;
            while p0 < k {
                let p_end = (p0 + BK).min(k);
                let mut j0 = 0;
                while j0 < n {
                    let j_end = (j0 + BJ).min(n);
                    let mut j = j0;
                    while j + 8 <= j_end {
                        let c00_ptr = c_base.add(i * n + j);
                        let c10_ptr = c_base.add((i + 1) * n + j);
                        let c20_ptr = c_base.add((i + 2) * n + j);
                        let c30_ptr = c_base.add((i + 3) * n + j);

                        let mut r00 = _mm_loadu_ps(c00_ptr);
                        let mut r01 = _mm_loadu_ps(c00_ptr.add(4));
                        let mut r10 = _mm_loadu_ps(c10_ptr);
                        let mut r11 = _mm_loadu_ps(c10_ptr.add(4));
                        let mut r20 = _mm_loadu_ps(c20_ptr);
                        let mut r21 = _mm_loadu_ps(c20_ptr.add(4));
                        let mut r30 = _mm_loadu_ps(c30_ptr);
                        let mut r31 = _mm_loadu_ps(c30_ptr.add(4));

                        for p in p0..p_end {
                            let bp = b_base.add(p * n + j);
                            let bv0 = _mm_loadu_ps(bp);
                            let bv1 = _mm_loadu_ps(bp.add(4));

                            let a0 = _mm_set1_ps(*a_base.add(i * k + p));
                            let a1 = _mm_set1_ps(*a_base.add((i + 1) * k + p));
                            let a2 = _mm_set1_ps(*a_base.add((i + 2) * k + p));
                            let a3 = _mm_set1_ps(*a_base.add((i + 3) * k + p));

                            r00 = _mm_add_ps(r00, _mm_mul_ps(a0, bv0));
                            r01 = _mm_add_ps(r01, _mm_mul_ps(a0, bv1));
                            r10 = _mm_add_ps(r10, _mm_mul_ps(a1, bv0));
                            r11 = _mm_add_ps(r11, _mm_mul_ps(a1, bv1));
                            r20 = _mm_add_ps(r20, _mm_mul_ps(a2, bv0));
                            r21 = _mm_add_ps(r21, _mm_mul_ps(a2, bv1));
                            r30 = _mm_add_ps(r30, _mm_mul_ps(a3, bv0));
                            r31 = _mm_add_ps(r31, _mm_mul_ps(a3, bv1));
                        }

                        _mm_storeu_ps(c00_ptr, r00);
                        _mm_storeu_ps(c00_ptr.add(4), r01);
                        _mm_storeu_ps(c10_ptr, r10);
                        _mm_storeu_ps(c10_ptr.add(4), r11);
                        _mm_storeu_ps(c20_ptr, r20);
                        _mm_storeu_ps(c20_ptr.add(4), r21);
                        _mm_storeu_ps(c30_ptr, r30);
                        _mm_storeu_ps(c30_ptr.add(4), r31);

                        j += 8;
                    }
                    for ii in i..i + 4 {
                        let c_row = c_base.add(ii * n);
                        for p in p0..p_end {
                            let a_ip = *a_base.add(ii * k + p);
                            let va = _mm_set1_ps(a_ip);
                            let b_row = b_base.add(p * n);
                            let mut jj = j;
                            while jj + 4 <= j_end {
                                let c0 = _mm_loadu_ps(c_row.add(jj));
                                let b0 = _mm_loadu_ps(b_row.add(jj));
                                _mm_storeu_ps(c_row.add(jj), _mm_add_ps(c0, _mm_mul_ps(va, b0)));
                                jj += 4;
                            }
                            while jj < j_end {
                                *c_row.add(jj) += a_ip * *b_row.add(jj);
                                jj += 1;
                            }
                        }
                    }
                    j0 = j_end;
                }
                p0 = p_end;
            }
            i += 4;
        }

        while i < m_rows {
            let c_row = c_base.add(i * n);
            for p in 0..k {
                let a_ip = *a_base.add(i * k + p);
                let va = _mm_set1_ps(a_ip);
                let b_row = b_base.add(p * n);
                let mut j = 0;
                while j + 4 <= n {
                    let c0 = _mm_loadu_ps(c_row.add(j));
                    let b0 = _mm_loadu_ps(b_row.add(j));
                    _mm_storeu_ps(c_row.add(j), _mm_add_ps(c0, _mm_mul_ps(va, b0)));
                    j += 4;
                }
                while j < n {
                    *c_row.add(j) += a_ip * *b_row.add(j);
                    j += 1;
                }
            }
            i += 1;
        }
    }

    #[inline(never)]
    pub fn dot(a: &Tensor, b: &Tensor) -> f32 {
        assert_eq!(a.len(), b.len(), "dot: mismatched lengths");
        #[cfg(target_arch = "x86_64")]
        unsafe {
            dot_impl(a.as_slice(), b.as_slice())
        }
        #[cfg(not(target_arch = "x86_64"))]
        super::scalar::dot(a, b)
    }

    #[inline(never)]
    pub fn axpy(alpha: f32, x: &Tensor, y: &mut Tensor) {
        assert_eq!(x.len(), y.len(), "axpy: mismatched lengths");
        #[cfg(target_arch = "x86_64")]
        unsafe {
            axpy_impl(alpha, x.as_slice(), y.as_mut_slice());
        }
        #[cfg(not(target_arch = "x86_64"))]
        super::scalar::axpy(alpha, x, y);
    }

    #[inline(never)]
    pub fn relu(x: &Tensor) -> Tensor {
        let mut out = Tensor::zeros(x.shape());
        #[cfg(target_arch = "x86_64")]
        unsafe {
            relu_impl(x.as_slice(), out.as_mut_slice());
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            out = super::scalar::relu(x);
        }
        out
    }

    #[inline(never)]
    pub fn matvec(a: &Tensor, x: &Tensor) -> Tensor {
        let (m, k) = a.dims_2d();
        assert_eq!(x.len(), k, "matvec: vector length {} != cols {}", x.len(), k);
        let mut y = Tensor::zeros(&[m]);
        #[cfg(target_arch = "x86_64")]
        unsafe {
            let xs = x.as_slice();
            let a_s = a.as_slice();
            for (row_idx, y_out) in y.as_mut_slice().iter_mut().enumerate() {
                *y_out = dot_impl(&a_s[row_idx * k..(row_idx + 1) * k], xs);
            }
        }
        #[cfg(not(target_arch = "x86_64"))]
        super::scalar::matvec_rows(a.as_slice(), x.as_slice(), y.as_mut_slice(), k);
        y
    }

    #[inline(never)]
    pub fn matmul(a: &Tensor, b: &Tensor) -> Tensor {
        let (m, k) = a.dims_2d();
        let (bk, n) = b.dims_2d();
        assert_eq!(k, bk, "matmul: inner dims {} != {}", k, bk);
        let mut c = Tensor::zeros(&[m, n]);
        #[cfg(target_arch = "x86_64")]
        unsafe {
            matmul_rows_impl(a.as_slice(), b.as_slice(), c.as_mut_slice(), k, n);
        }
        #[cfg(not(target_arch = "x86_64"))]
        super::scalar::matmul_rows(a.as_slice(), b.as_slice(), c.as_mut_slice(), k, n);
        c
    }
}

/// Explicit x86_64 AVX2 + FMA kernels (256-bit SIMD, 8 x f32 per register).
pub mod avx2 {
    use super::Tensor;

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2", enable = "fma")]
    pub(crate) unsafe fn dot_slice(xs: &[f32], ys: &[f32]) -> f32 {
        use std::arch::x86_64::*;
        let n = xs.len();
        let mut i = 0;
        let mut acc0 = _mm256_setzero_ps();
        let mut acc1 = _mm256_setzero_ps();
        let mut acc2 = _mm256_setzero_ps();
        let mut acc3 = _mm256_setzero_ps();
        while i + 32 <= n {
            acc0 = _mm256_fmadd_ps(
                _mm256_loadu_ps(xs.as_ptr().add(i)),
                _mm256_loadu_ps(ys.as_ptr().add(i)),
                acc0,
            );
            acc1 = _mm256_fmadd_ps(
                _mm256_loadu_ps(xs.as_ptr().add(i + 8)),
                _mm256_loadu_ps(ys.as_ptr().add(i + 8)),
                acc1,
            );
            acc2 = _mm256_fmadd_ps(
                _mm256_loadu_ps(xs.as_ptr().add(i + 16)),
                _mm256_loadu_ps(ys.as_ptr().add(i + 16)),
                acc2,
            );
            acc3 = _mm256_fmadd_ps(
                _mm256_loadu_ps(xs.as_ptr().add(i + 24)),
                _mm256_loadu_ps(ys.as_ptr().add(i + 24)),
                acc3,
            );
            i += 32;
        }
        let mut acc = _mm256_add_ps(_mm256_add_ps(acc0, acc1), _mm256_add_ps(acc2, acc3));
        while i + 8 <= n {
            acc = _mm256_fmadd_ps(
                _mm256_loadu_ps(xs.as_ptr().add(i)),
                _mm256_loadu_ps(ys.as_ptr().add(i)),
                acc,
            );
            i += 8;
        }
        let mut tmp = [0.0_f32; 8];
        _mm256_storeu_ps(tmp.as_mut_ptr(), acc);
        let mut sum = ((tmp[0] + tmp[1]) + (tmp[2] + tmp[3]))
            + ((tmp[4] + tmp[5]) + (tmp[6] + tmp[7]));
        while i < n {
            sum += xs[i] * ys[i];
            i += 1;
        }
        sum
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2", enable = "fma")]
    pub(crate) unsafe fn axpy_slice(alpha: f32, xs: &[f32], ys: &mut [f32]) {
        use std::arch::x86_64::*;
        let n = xs.len();
        let va = _mm256_set1_ps(alpha);
        let mut i = 0;
        while i + 16 <= n {
            let x0 = _mm256_loadu_ps(xs.as_ptr().add(i));
            let y0 = _mm256_loadu_ps(ys.as_ptr().add(i));
            let x1 = _mm256_loadu_ps(xs.as_ptr().add(i + 8));
            let y1 = _mm256_loadu_ps(ys.as_ptr().add(i + 8));
            _mm256_storeu_ps(ys.as_mut_ptr().add(i), _mm256_fmadd_ps(va, x0, y0));
            _mm256_storeu_ps(ys.as_mut_ptr().add(i + 8), _mm256_fmadd_ps(va, x1, y1));
            i += 16;
        }
        while i + 8 <= n {
            let x0 = _mm256_loadu_ps(xs.as_ptr().add(i));
            let y0 = _mm256_loadu_ps(ys.as_ptr().add(i));
            _mm256_storeu_ps(ys.as_mut_ptr().add(i), _mm256_fmadd_ps(va, x0, y0));
            i += 8;
        }
        while i < n {
            ys[i] += alpha * xs[i];
            i += 1;
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    pub(crate) unsafe fn relu_slice(xs: &[f32], os: &mut [f32]) {
        use std::arch::x86_64::*;
        let n = xs.len();
        let zero = _mm256_setzero_ps();
        let mut i = 0;
        while i + 16 <= n {
            let x0 = _mm256_loadu_ps(xs.as_ptr().add(i));
            let x1 = _mm256_loadu_ps(xs.as_ptr().add(i + 8));
            _mm256_storeu_ps(os.as_mut_ptr().add(i), _mm256_max_ps(x0, zero));
            _mm256_storeu_ps(os.as_mut_ptr().add(i + 8), _mm256_max_ps(x1, zero));
            i += 16;
        }
        while i + 8 <= n {
            let x0 = _mm256_loadu_ps(xs.as_ptr().add(i));
            _mm256_storeu_ps(os.as_mut_ptr().add(i), _mm256_max_ps(x0, zero));
            i += 8;
        }
        while i < n {
            os[i] = xs[i].max(0.0);
            i += 1;
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2", enable = "fma")]
    pub(crate) unsafe fn matvec_rows(a_rows: &[f32], x: &[f32], y_rows: &mut [f32], k: usize) {
        for (row_idx, y_out) in y_rows.iter_mut().enumerate() {
            *y_out = dot_slice(&a_rows[row_idx * k..(row_idx + 1) * k], x);
        }
    }

    /// Cache-blocked 4-row micro-kernel GEMM with AVX2 + FMA.
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2", enable = "fma")]
    pub(crate) unsafe fn matmul_rows(a_rows: &[f32], b: &[f32], c_rows: &mut [f32], k: usize, n: usize) {
        use std::arch::x86_64::*;
        const BK: usize = 64;
        const BJ: usize = 128;

        let m_rows = c_rows.len() / n;
        let a_base = a_rows.as_ptr();
        let b_base = b.as_ptr();
        let c_base = c_rows.as_mut_ptr();

        let mut i = 0;
        while i + 4 <= m_rows {
            let mut p0 = 0;
            while p0 < k {
                let p_end = (p0 + BK).min(k);
                let mut j0 = 0;
                while j0 < n {
                    let j_end = (j0 + BJ).min(n);
                    let mut j = j0;
                    while j + 16 <= j_end {
                        let c00_ptr = c_base.add(i * n + j);
                        let c10_ptr = c_base.add((i + 1) * n + j);
                        let c20_ptr = c_base.add((i + 2) * n + j);
                        let c30_ptr = c_base.add((i + 3) * n + j);

                        let mut r00 = _mm256_loadu_ps(c00_ptr);
                        let mut r01 = _mm256_loadu_ps(c00_ptr.add(8));
                        let mut r10 = _mm256_loadu_ps(c10_ptr);
                        let mut r11 = _mm256_loadu_ps(c10_ptr.add(8));
                        let mut r20 = _mm256_loadu_ps(c20_ptr);
                        let mut r21 = _mm256_loadu_ps(c20_ptr.add(8));
                        let mut r30 = _mm256_loadu_ps(c30_ptr);
                        let mut r31 = _mm256_loadu_ps(c30_ptr.add(8));

                        for p in p0..p_end {
                            let bp = b_base.add(p * n + j);
                            let bv0 = _mm256_loadu_ps(bp);
                            let bv1 = _mm256_loadu_ps(bp.add(8));

                            let a0 = _mm256_set1_ps(*a_base.add(i * k + p));
                            let a1 = _mm256_set1_ps(*a_base.add((i + 1) * k + p));
                            let a2 = _mm256_set1_ps(*a_base.add((i + 2) * k + p));
                            let a3 = _mm256_set1_ps(*a_base.add((i + 3) * k + p));

                            r00 = _mm256_fmadd_ps(a0, bv0, r00);
                            r01 = _mm256_fmadd_ps(a0, bv1, r01);
                            r10 = _mm256_fmadd_ps(a1, bv0, r10);
                            r11 = _mm256_fmadd_ps(a1, bv1, r11);
                            r20 = _mm256_fmadd_ps(a2, bv0, r20);
                            r21 = _mm256_fmadd_ps(a2, bv1, r21);
                            r30 = _mm256_fmadd_ps(a3, bv0, r30);
                            r31 = _mm256_fmadd_ps(a3, bv1, r31);
                        }

                        _mm256_storeu_ps(c00_ptr, r00);
                        _mm256_storeu_ps(c00_ptr.add(8), r01);
                        _mm256_storeu_ps(c10_ptr, r10);
                        _mm256_storeu_ps(c10_ptr.add(8), r11);
                        _mm256_storeu_ps(c20_ptr, r20);
                        _mm256_storeu_ps(c20_ptr.add(8), r21);
                        _mm256_storeu_ps(c30_ptr, r30);
                        _mm256_storeu_ps(c30_ptr.add(8), r31);

                        j += 16;
                    }
                    // Remaining columns inside this block
                    for ii in i..i + 4 {
                        let c_row = c_base.add(ii * n);
                        for p in p0..p_end {
                            let a_ip = *a_base.add(ii * k + p);
                            let va = _mm256_set1_ps(a_ip);
                            let b_row = b_base.add(p * n);
                            let mut jj = j;
                            while jj + 8 <= j_end {
                                let vc = _mm256_loadu_ps(c_row.add(jj));
                                let vb = _mm256_loadu_ps(b_row.add(jj));
                                _mm256_storeu_ps(c_row.add(jj), _mm256_fmadd_ps(va, vb, vc));
                                jj += 8;
                            }
                            while jj < j_end {
                                *c_row.add(jj) += a_ip * *b_row.add(jj);
                                jj += 1;
                            }
                        }
                    }
                    j0 = j_end;
                }
                p0 = p_end;
            }
            i += 4;
        }

        // Remaining tail rows (< 4)
        while i < m_rows {
            let c_row = c_base.add(i * n);
            for p in 0..k {
                let a_ip = *a_base.add(i * k + p);
                let va = _mm256_set1_ps(a_ip);
                let b_row = b_base.add(p * n);
                let mut j = 0;
                while j + 8 <= n {
                    let vc = _mm256_loadu_ps(c_row.add(j));
                    let vb = _mm256_loadu_ps(b_row.add(j));
                    _mm256_storeu_ps(c_row.add(j), _mm256_fmadd_ps(va, vb, vc));
                    j += 8;
                }
                while j < n {
                    *c_row.add(j) += a_ip * *b_row.add(j);
                    j += 1;
                }
            }
            i += 1;
        }
    }

    #[inline(never)]
    pub fn dot(a: &Tensor, b: &Tensor) -> f32 {
        assert_eq!(a.len(), b.len(), "dot: mismatched lengths");
        #[cfg(target_arch = "x86_64")]
        unsafe {
            dot_slice(a.as_slice(), b.as_slice())
        }
        #[cfg(not(target_arch = "x86_64"))]
        super::scalar::dot(a, b)
    }

    #[inline(never)]
    pub fn axpy(alpha: f32, x: &Tensor, y: &mut Tensor) {
        assert_eq!(x.len(), y.len(), "axpy: mismatched lengths");
        #[cfg(target_arch = "x86_64")]
        unsafe {
            axpy_slice(alpha, x.as_slice(), y.as_mut_slice());
        }
        #[cfg(not(target_arch = "x86_64"))]
        super::scalar::axpy(alpha, x, y);
    }

    #[inline(never)]
    pub fn relu(x: &Tensor) -> Tensor {
        let mut out = Tensor::zeros(x.shape());
        #[cfg(target_arch = "x86_64")]
        unsafe {
            relu_slice(x.as_slice(), out.as_mut_slice());
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            out = super::scalar::relu(x);
        }
        out
    }

    #[inline(never)]
    pub fn matvec(a: &Tensor, x: &Tensor) -> Tensor {
        let (m, k) = a.dims_2d();
        assert_eq!(x.len(), k, "matvec: vector length {} != cols {}", x.len(), k);
        let mut y = Tensor::zeros(&[m]);
        #[cfg(target_arch = "x86_64")]
        unsafe {
            matvec_rows(a.as_slice(), x.as_slice(), y.as_mut_slice(), k);
        }
        #[cfg(not(target_arch = "x86_64"))]
        super::scalar::matvec_rows(a.as_slice(), x.as_slice(), y.as_mut_slice(), k);
        y
    }

    #[inline(never)]
    pub fn matmul(a: &Tensor, b: &Tensor) -> Tensor {
        let (m, k) = a.dims_2d();
        let (bk, n) = b.dims_2d();
        assert_eq!(k, bk, "matmul: inner dims {} != {}", k, bk);
        let mut c = Tensor::zeros(&[m, n]);
        #[cfg(target_arch = "x86_64")]
        unsafe {
            matmul_rows(a.as_slice(), b.as_slice(), c.as_mut_slice(), k, n);
        }
        #[cfg(not(target_arch = "x86_64"))]
        super::scalar::matmul_rows(a.as_slice(), b.as_slice(), c.as_mut_slice(), k, n);
        c
    }
}

/// Explicit x86_64 AVX-512F kernels (512-bit SIMD, 16 x f32 per register).
pub mod avx512 {
    use super::Tensor;

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx512f")]
    pub(crate) unsafe fn dot_slice(xs: &[f32], ys: &[f32]) -> f32 {
        use std::arch::x86_64::*;
        let n = xs.len();
        let mut i = 0;
        let mut acc0 = _mm512_setzero_ps();
        let mut acc1 = _mm512_setzero_ps();
        while i + 32 <= n {
            acc0 = _mm512_fmadd_ps(
                _mm512_loadu_ps(xs.as_ptr().add(i)),
                _mm512_loadu_ps(ys.as_ptr().add(i)),
                acc0,
            );
            acc1 = _mm512_fmadd_ps(
                _mm512_loadu_ps(xs.as_ptr().add(i + 16)),
                _mm512_loadu_ps(ys.as_ptr().add(i + 16)),
                acc1,
            );
            i += 32;
        }
        let mut acc = _mm512_add_ps(acc0, acc1);
        while i + 16 <= n {
            acc = _mm512_fmadd_ps(
                _mm512_loadu_ps(xs.as_ptr().add(i)),
                _mm512_loadu_ps(ys.as_ptr().add(i)),
                acc,
            );
            i += 16;
        }
        let mut sum = _mm512_reduce_add_ps(acc);
        while i < n {
            sum += xs[i] * ys[i];
            i += 1;
        }
        sum
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx512f")]
    pub(crate) unsafe fn axpy_slice(alpha: f32, xs: &[f32], ys: &mut [f32]) {
        use std::arch::x86_64::*;
        let n = xs.len();
        let va = _mm512_set1_ps(alpha);
        let mut i = 0;
        while i + 16 <= n {
            let vx = _mm512_loadu_ps(xs.as_ptr().add(i));
            let vy = _mm512_loadu_ps(ys.as_ptr().add(i));
            _mm512_storeu_ps(ys.as_mut_ptr().add(i), _mm512_fmadd_ps(va, vx, vy));
            i += 16;
        }
        while i < n {
            ys[i] += alpha * xs[i];
            i += 1;
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx512f")]
    pub(crate) unsafe fn relu_slice(xs: &[f32], os: &mut [f32]) {
        use std::arch::x86_64::*;
        let n = xs.len();
        let zero = _mm512_setzero_ps();
        let mut i = 0;
        while i + 16 <= n {
            let vx = _mm512_loadu_ps(xs.as_ptr().add(i));
            _mm512_storeu_ps(os.as_mut_ptr().add(i), _mm512_max_ps(vx, zero));
            i += 16;
        }
        while i < n {
            os[i] = xs[i].max(0.0);
            i += 1;
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx512f")]
    pub(crate) unsafe fn matvec_rows(a_rows: &[f32], x: &[f32], y_rows: &mut [f32], k: usize) {
        for (row_idx, y_out) in y_rows.iter_mut().enumerate() {
            *y_out = dot_slice(&a_rows[row_idx * k..(row_idx + 1) * k], x);
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx512f")]
    pub(crate) unsafe fn matmul_rows(a_rows: &[f32], b: &[f32], c_rows: &mut [f32], k: usize, n: usize) {
        use std::arch::x86_64::*;
        let m_rows = c_rows.len() / n;
        let a_base = a_rows.as_ptr();
        let b_base = b.as_ptr();
        let c_base = c_rows.as_mut_ptr();

        for i in 0..m_rows {
            let c_row = c_base.add(i * n);
            for p in 0..k {
                let a_ip = *a_base.add(i * k + p);
                let va = _mm512_set1_ps(a_ip);
                let b_row = b_base.add(p * n);
                let mut j = 0;
                while j + 16 <= n {
                    let vc = _mm512_loadu_ps(c_row.add(j));
                    let vb = _mm512_loadu_ps(b_row.add(j));
                    _mm512_storeu_ps(c_row.add(j), _mm512_fmadd_ps(va, vb, vc));
                    j += 16;
                }
                while j < n {
                    *c_row.add(j) += a_ip * *b_row.add(j);
                    j += 1;
                }
            }
        }
    }

    #[inline(never)]
    pub fn dot(a: &Tensor, b: &Tensor) -> f32 {
        assert_eq!(a.len(), b.len(), "dot: mismatched lengths");
        #[cfg(target_arch = "x86_64")]
        unsafe {
            dot_slice(a.as_slice(), b.as_slice())
        }
        #[cfg(not(target_arch = "x86_64"))]
        super::scalar::dot(a, b)
    }

    #[inline(never)]
    pub fn axpy(alpha: f32, x: &Tensor, y: &mut Tensor) {
        assert_eq!(x.len(), y.len(), "axpy: mismatched lengths");
        #[cfg(target_arch = "x86_64")]
        unsafe {
            axpy_slice(alpha, x.as_slice(), y.as_mut_slice());
        }
        #[cfg(not(target_arch = "x86_64"))]
        super::scalar::axpy(alpha, x, y);
    }

    #[inline(never)]
    pub fn relu(x: &Tensor) -> Tensor {
        let mut out = Tensor::zeros(x.shape());
        #[cfg(target_arch = "x86_64")]
        unsafe {
            relu_slice(x.as_slice(), out.as_mut_slice());
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            out = super::scalar::relu(x);
        }
        out
    }

    #[inline(never)]
    pub fn matvec(a: &Tensor, x: &Tensor) -> Tensor {
        let (m, k) = a.dims_2d();
        assert_eq!(x.len(), k, "matvec: vector length {} != cols {}", x.len(), k);
        let mut y = Tensor::zeros(&[m]);
        #[cfg(target_arch = "x86_64")]
        unsafe {
            matvec_rows(a.as_slice(), x.as_slice(), y.as_mut_slice(), k);
        }
        #[cfg(not(target_arch = "x86_64"))]
        super::scalar::matvec_rows(a.as_slice(), x.as_slice(), y.as_mut_slice(), k);
        y
    }

    #[inline(never)]
    pub fn matmul(a: &Tensor, b: &Tensor) -> Tensor {
        let (m, k) = a.dims_2d();
        let (bk, n) = b.dims_2d();
        assert_eq!(k, bk, "matmul: inner dims {} != {}", k, bk);
        let mut c = Tensor::zeros(&[m, n]);
        #[cfg(target_arch = "x86_64")]
        unsafe {
            matmul_rows(a.as_slice(), b.as_slice(), c.as_mut_slice(), k, n);
        }
        #[cfg(not(target_arch = "x86_64"))]
        super::scalar::matmul_rows(a.as_slice(), b.as_slice(), c.as_mut_slice(), k, n);
        c
    }
}

/// Low-overhead persistent fork-join thread pool (zero external dependencies),
/// parameterized over scalar, AVX2+FMA, or AVX-512 per-chunk kernels.
pub mod multicore {
    use std::cell::UnsafeCell;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Condvar, Mutex, OnceLock};

    use super::{Tensor, worker_count};

    type TaskFn = dyn Fn(usize, usize) + Sync + 'static;

    struct JobSlot {
        active_workers: usize,
        task: Option<*const TaskFn>,
    }

    struct PoolShared {
        epoch: AtomicUsize,
        done_count: AtomicUsize,
        slot: UnsafeCell<JobSlot>,
        wake_mu: Mutex<usize>,
        wake_cv: Condvar,
    }

    // Safety: `slot` is written strictly by the main thread while holding `run_lock` and
    // before publishing `epoch` with Release ordering, and read by workers only after
    // observing the new `epoch` with Acquire ordering and before incrementing `done_count`.
    unsafe impl Send for PoolShared {}
    unsafe impl Sync for PoolShared {}

    struct ParallelPool {
        num_threads: usize,
        run_lock: Mutex<()>,
        shared: Arc<PoolShared>,
    }

    impl ParallelPool {
        fn new(num_threads: usize) -> Self {
            let num_threads = num_threads.max(1);
            let shared = Arc::new(PoolShared {
                epoch: AtomicUsize::new(0),
                done_count: AtomicUsize::new(0),
                slot: UnsafeCell::new(JobSlot {
                    active_workers: 0,
                    task: None,
                }),
                wake_mu: Mutex::new(0),
                wake_cv: Condvar::new(),
            });

            for tid in 1..num_threads {
                let s = Arc::clone(&shared);
                std::thread::Builder::new()
                    .name(format!("tensor-worker-{tid}"))
                    .spawn(move || {
                        let mut last_epoch = 0usize;
                        loop {
                            let mut cur = s.epoch.load(Ordering::Acquire);
                            if cur == last_epoch {
                                for _ in 0..4096 {
                                    std::hint::spin_loop();
                                    cur = s.epoch.load(Ordering::Acquire);
                                    if cur != last_epoch {
                                        break;
                                    }
                                }
                            }
                            if cur == last_epoch {
                                let mut guard = s.wake_mu.lock().unwrap();
                                while *guard == last_epoch {
                                    guard = s.wake_cv.wait(guard).unwrap();
                                }
                                cur = *guard;
                                drop(guard);
                                // Pair with main thread's Release store on `epoch`.
                                let _ = s.epoch.load(Ordering::Acquire);
                            }

                            last_epoch = cur;
                            let (active, task_opt) = unsafe {
                                let slot = &*s.slot.get();
                                (slot.active_workers, slot.task)
                            };
                            if tid < active {
                                if let Some(ptr) = task_opt {
                                    let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
                                        (&*ptr)(tid, active);
                                    }));
                                }
                                s.done_count.fetch_add(1, Ordering::Release);
                            }
                        }
                    })
                    .expect("failed to spawn tensor worker thread");
            }

            Self {
                num_threads,
                run_lock: Mutex::new(()),
                shared,
            }
        }

        fn parallel_for<F>(&self, tasks: usize, f: F)
        where
            F: Fn(usize, usize) + Sync,
        {
            let active = tasks.min(self.num_threads);
            if active <= 1 {
                f(0, 1);
                return;
            }

            let _run_guard = self.run_lock.lock().unwrap();
            let trait_obj: &(dyn Fn(usize, usize) + Sync) = &f;
            // Safety: lifetime erasure is bounded by the barrier below, which guarantees all
            // workers finish accessing `trait_obj` before `parallel_for` returns.
            let erased: *const TaskFn = unsafe { std::mem::transmute(trait_obj) };

            self.shared.done_count.store(0, Ordering::Release);
            unsafe {
                let slot = &mut *self.shared.slot.get();
                slot.active_workers = active;
                slot.task = Some(erased);
            }

            {
                let mut guard = self.shared.wake_mu.lock().unwrap();
                *guard = guard.wrapping_add(1);
                let next_epoch = *guard;
                self.shared.epoch.store(next_epoch, Ordering::Release);
            }
            self.shared.wake_cv.notify_all();

            // Main thread participates as worker 0; ensure workers join even if worker 0 panics.
            let main_res = catch_unwind(AssertUnwindSafe(|| {
                f(0, active);
            }));

            let expected = active - 1;
            while self.shared.done_count.load(Ordering::Acquire) < expected {
                std::hint::spin_loop();
            }

            unsafe {
                let slot = &mut *self.shared.slot.get();
                slot.task = None;
                slot.active_workers = 0;
            }

            if let Err(payload) = main_res {
                std::panic::resume_unwind(payload);
            }
        }
    }

    fn pool() -> &'static ParallelPool {
        static POOL: OnceLock<ParallelPool> = OnceLock::new();
        POOL.get_or_init(|| ParallelPool::new(worker_count()))
    }

    #[derive(Clone, Copy)]
    struct SendMutPtr<T>(*mut T);
    impl<T> SendMutPtr<T> {
        #[inline(always)]
        fn ptr(self) -> *mut T {
            self.0
        }
    }
    unsafe impl<T: Send> Send for SendMutPtr<T> {}
    unsafe impl<T: Send> Sync for SendMutPtr<T> {}

    fn parallel_dot<F>(a: &Tensor, b: &Tensor, chunk_dot: F) -> f32
    where
        F: Fn(&[f32], &[f32]) -> f32 + Sync,
    {
        assert_eq!(a.len(), b.len(), "dot: mismatched lengths");
        let n = a.len();
        let threads = worker_count().min(n.max(1));
        if threads <= 1 || n < 65_536 {
            return chunk_dot(a.as_slice(), b.as_slice());
        }
        let (xs, ys) = (a.as_slice(), b.as_slice());
        let mut partials = vec![0.0_f32; threads];
        let p_ptr = SendMutPtr(partials.as_mut_ptr());
        pool().parallel_for(threads, |tid, active| {
            let chunk = n.div_ceil(active);
            let start = tid * chunk;
            if start < n {
                let end = (start + chunk).min(n);
                let val = chunk_dot(&xs[start..end], &ys[start..end]);
                unsafe {
                    *p_ptr.ptr().add(tid) = val;
                }
            }
        });
        partials.into_iter().sum()
    }

    fn parallel_axpy<F>(alpha: f32, x: &Tensor, y: &mut Tensor, chunk_axpy: F)
    where
        F: Fn(f32, &[f32], &mut [f32]) + Sync,
    {
        assert_eq!(x.len(), y.len(), "axpy: mismatched lengths");
        let n = x.len();
        let threads = worker_count().min(n.max(1));
        if threads <= 1 || n < 65_536 {
            chunk_axpy(alpha, x.as_slice(), y.as_mut_slice());
            return;
        }
        let xs = x.as_slice();
        let y_ptr = SendMutPtr(y.as_mut_slice().as_mut_ptr());
        pool().parallel_for(threads, |tid, active| {
            let chunk = n.div_ceil(active);
            let start = tid * chunk;
            if start < n {
                let end = (start + chunk).min(n);
                let y_chunk = unsafe {
                    std::slice::from_raw_parts_mut(y_ptr.ptr().add(start), end - start)
                };
                chunk_axpy(alpha, &xs[start..end], y_chunk);
            }
        });
    }

    fn parallel_relu<F>(x: &Tensor, chunk_relu: F) -> Tensor
    where
        F: Fn(&[f32], &mut [f32]) + Sync,
    {
        let n = x.len();
        let mut out = Tensor::zeros(x.shape());
        let threads = worker_count().min(n.max(1));
        if threads <= 1 || n < 65_536 {
            chunk_relu(x.as_slice(), out.as_mut_slice());
            return out;
        }
        let xs = x.as_slice();
        let o_ptr = SendMutPtr(out.as_mut_slice().as_mut_ptr());
        pool().parallel_for(threads, |tid, active| {
            let chunk = n.div_ceil(active);
            let start = tid * chunk;
            if start < n {
                let end = (start + chunk).min(n);
                let o_chunk = unsafe {
                    std::slice::from_raw_parts_mut(o_ptr.ptr().add(start), end - start)
                };
                chunk_relu(&xs[start..end], o_chunk);
            }
        });
        out
    }

    fn parallel_matvec<F>(a: &Tensor, x: &Tensor, rows_kernel: F) -> Tensor
    where
        F: Fn(&[f32], &[f32], &mut [f32], usize) + Sync,
    {
        let (m, k) = a.dims_2d();
        assert_eq!(x.len(), k, "matvec: vector length {} != cols {}", x.len(), k);
        let mut y = Tensor::zeros(&[m]);
        let threads = worker_count().min(m.max(1));
        if threads <= 1 || m * k < 65_536 {
            rows_kernel(a.as_slice(), x.as_slice(), y.as_mut_slice(), k);
            return y;
        }
        let a_s = a.as_slice();
        let x_s = x.as_slice();
        let y_ptr = SendMutPtr(y.as_mut_slice().as_mut_ptr());
        pool().parallel_for(threads, |tid, active| {
            let rows_per_thread = m.div_ceil(active);
            let r_start = tid * rows_per_thread;
            if r_start < m {
                let r_end = (r_start + rows_per_thread).min(m);
                let r_count = r_end - r_start;
                let a_chunk = &a_s[r_start * k..r_end * k];
                let y_chunk = unsafe {
                    std::slice::from_raw_parts_mut(y_ptr.ptr().add(r_start), r_count)
                };
                rows_kernel(a_chunk, x_s, y_chunk, k);
            }
        });
        y
    }

    fn parallel_matmul<F>(a: &Tensor, b: &Tensor, rows_kernel: F) -> Tensor
    where
        F: Fn(&[f32], &[f32], &mut [f32], usize, usize) + Sync,
    {
        let (m, k) = a.dims_2d();
        let (bk, n) = b.dims_2d();
        assert_eq!(k, bk, "matmul: inner dims {} != {}", k, bk);
        let mut c = Tensor::zeros(&[m, n]);
        let threads = worker_count().min(m.max(1));
        if threads <= 1 || m < 16 {
            rows_kernel(a.as_slice(), b.as_slice(), c.as_mut_slice(), k, n);
            return c;
        }
        // Align chunk size to 4 rows to maximize the 4-row SIMD register micro-kernel
        let a_s = a.as_slice();
        let b_s = b.as_slice();
        let c_ptr = SendMutPtr(c.as_mut_slice().as_mut_ptr());
        pool().parallel_for(threads, |tid, active| {
            let rows_per_thread = (m.div_ceil(active).div_ceil(4) * 4).max(4);
            let r_start = tid * rows_per_thread;
            if r_start < m {
                let r_end = (r_start + rows_per_thread).min(m);
                let r_count = r_end - r_start;
                let a_chunk = &a_s[r_start * k..r_end * k];
                let c_chunk = unsafe {
                    std::slice::from_raw_parts_mut(c_ptr.ptr().add(r_start * n), r_count * n)
                };
                rows_kernel(a_chunk, b_s, c_chunk, k, n);
            }
        });
        c
    }

    pub mod scalar_mt {
        use super::*;
        use crate::kernels::scalar;

        #[inline(never)]
        pub fn dot(a: &Tensor, b: &Tensor) -> f32 {
            parallel_dot(a, b, |xs, ys| {
                xs.iter().zip(ys.iter()).map(|(x, y)| x * y).sum()
            })
        }

        #[inline(never)]
        pub fn axpy(alpha: f32, x: &Tensor, y: &mut Tensor) {
            parallel_axpy(alpha, x, y, |al, xs, ys| {
                for i in 0..xs.len() {
                    ys[i] += al * xs[i];
                }
            });
        }

        #[inline(never)]
        pub fn relu(x: &Tensor) -> Tensor {
            parallel_relu(x, |xs, os| {
                for i in 0..xs.len() {
                    os[i] = if xs[i] > 0.0 { xs[i] } else { 0.0 };
                }
            })
        }

        #[inline(never)]
        pub fn matvec(a: &Tensor, x: &Tensor) -> Tensor {
            parallel_matvec(a, x, scalar::matvec_rows)
        }

        #[inline(never)]
        pub fn matmul(a: &Tensor, b: &Tensor) -> Tensor {
            parallel_matmul(a, b, scalar::matmul_rows)
        }
    }

    pub mod avx2_mt {
        use super::*;

        #[inline(never)]
        pub fn dot(a: &Tensor, b: &Tensor) -> f32 {
            #[cfg(target_arch = "x86_64")]
            {
                parallel_dot(a, b, |xs, ys| unsafe {
                    crate::kernels::avx2::dot_slice(xs, ys)
                })
            }
            #[cfg(not(target_arch = "x86_64"))]
            scalar_mt::dot(a, b)
        }

        #[inline(never)]
        pub fn axpy(alpha: f32, x: &Tensor, y: &mut Tensor) {
            #[cfg(target_arch = "x86_64")]
            parallel_axpy(alpha, x, y, |al, xs, ys| unsafe {
                crate::kernels::avx2::axpy_slice(al, xs, ys);
            });
            #[cfg(not(target_arch = "x86_64"))]
            scalar_mt::axpy(alpha, x, y);
        }

        #[inline(never)]
        pub fn relu(x: &Tensor) -> Tensor {
            #[cfg(target_arch = "x86_64")]
            {
                parallel_relu(x, |xs, os| unsafe {
                    crate::kernels::avx2::relu_slice(xs, os);
                })
            }
            #[cfg(not(target_arch = "x86_64"))]
            scalar_mt::relu(x)
        }

        #[inline(never)]
        pub fn matvec(a: &Tensor, x: &Tensor) -> Tensor {
            #[cfg(target_arch = "x86_64")]
            {
                parallel_matvec(a, x, |ar, xs, yr, k| unsafe {
                    crate::kernels::avx2::matvec_rows(ar, xs, yr, k);
                })
            }
            #[cfg(not(target_arch = "x86_64"))]
            scalar_mt::matvec(a, x)
        }

        #[inline(never)]
        pub fn matmul(a: &Tensor, b: &Tensor) -> Tensor {
            #[cfg(target_arch = "x86_64")]
            {
                parallel_matmul(a, b, |ar, bs, cr, k, n| unsafe {
                    crate::kernels::avx2::matmul_rows(ar, bs, cr, k, n);
                })
            }
            #[cfg(not(target_arch = "x86_64"))]
            scalar_mt::matmul(a, b)
        }
    }

    pub mod avx512_mt {
        use super::*;

        #[inline(never)]
        pub fn dot(a: &Tensor, b: &Tensor) -> f32 {
            #[cfg(target_arch = "x86_64")]
            {
                parallel_dot(a, b, |xs, ys| unsafe {
                    crate::kernels::avx512::dot_slice(xs, ys)
                })
            }
            #[cfg(not(target_arch = "x86_64"))]
            scalar_mt::dot(a, b)
        }

        #[inline(never)]
        pub fn axpy(alpha: f32, x: &Tensor, y: &mut Tensor) {
            #[cfg(target_arch = "x86_64")]
            parallel_axpy(alpha, x, y, |al, xs, ys| unsafe {
                crate::kernels::avx512::axpy_slice(al, xs, ys);
            });
            #[cfg(not(target_arch = "x86_64"))]
            scalar_mt::axpy(alpha, x, y);
        }

        #[inline(never)]
        pub fn relu(x: &Tensor) -> Tensor {
            #[cfg(target_arch = "x86_64")]
            {
                parallel_relu(x, |xs, os| unsafe {
                    crate::kernels::avx512::relu_slice(xs, os);
                })
            }
            #[cfg(not(target_arch = "x86_64"))]
            scalar_mt::relu(x)
        }

        #[inline(never)]
        pub fn matvec(a: &Tensor, x: &Tensor) -> Tensor {
            #[cfg(target_arch = "x86_64")]
            {
                parallel_matvec(a, x, |ar, xs, yr, k| unsafe {
                    crate::kernels::avx512::matvec_rows(ar, xs, yr, k);
                })
            }
            #[cfg(not(target_arch = "x86_64"))]
            scalar_mt::matvec(a, x)
        }

        #[inline(never)]
        pub fn matmul(a: &Tensor, b: &Tensor) -> Tensor {
            #[cfg(target_arch = "x86_64")]
            {
                parallel_matmul(a, b, |ar, bs, cr, k, n| unsafe {
                    crate::kernels::avx512::matmul_rows(ar, bs, cr, k, n);
                })
            }
            #[cfg(not(target_arch = "x86_64"))]
            scalar_mt::matmul(a, b)
        }
    }
}
