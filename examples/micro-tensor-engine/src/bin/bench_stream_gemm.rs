//! Roofline & absolute HPC reference benchmark:
//!   1. STREAM Triad (L3-resident & DRAM-resident, 1 thread and 8 threads)
//!   2. `gemm` v0.18.2 reference SGEMM (single-thread & 8-thread) vs. our AVX2+FMA vs. pure AVX2 (no FMA)
//!   3. NVIDIA RTX 2070 CUDA PTX SGEMM breakdown (kernel-only VRAM vs. PCIe end-to-end)

use std::hint::black_box;
use std::time::Instant;

use gemm::{Parallelism, gemm};
use micro_tensor_engine::cuda_backend;
use micro_tensor_engine::kernels::{avx2, multicore, worker_count};
use micro_tensor_engine::Tensor;

fn median_iqr(samples: &mut [f64]) -> (f64, f64) {
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = samples.len();
    let med = if n % 2 == 0 {
        0.5 * (samples[n / 2 - 1] + samples[n / 2])
    } else {
        samples[n / 2]
    };
    let q1 = samples[n / 4];
    let q3 = samples[(3 * n) / 4];
    (med, q3 - q1)
}

fn time_samples<F: FnMut()>(warmup: usize, iters: usize, mut f: F) -> Vec<f64> {
    for _ in 0..warmup {
        f();
    }
    let mut s = Vec::with_capacity(iters);
    for _ in 0..iters {
        let t0 = Instant::now();
        f();
        s.push(t0.elapsed().as_secs_f64() * 1_000.0);
    }
    s
}

fn run_stream_triad(name: &str, n: usize, multi: bool, warmup: usize, iters: usize) {
    let mut a = vec![0.0_f32; n];
    let b = vec![1.5_f32; n];
    let c = vec![2.5_f32; n];
    let scalar = 3.0_f32;
    // Touch all pages first to avoid lazy mmap page-fault artifacts
    for v in &mut a {
        *v = 0.0;
    }

    let mut samples = time_samples(warmup, iters, || {
        if multi {
            multicore::stream_triad_mt(black_box(&mut a), black_box(&b), black_box(&c), scalar);
        } else {
            #[cfg(target_arch = "x86_64")]
            if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
                unsafe {
                    multicore::stream_triad_avx2(black_box(&mut a), black_box(&b), black_box(&c), scalar);
                }
            } else {
                for i in 0..n {
                    a[i] = b[i] + scalar * c[i];
                }
            }
            #[cfg(not(target_arch = "x86_64"))]
            for i in 0..n {
                a[i] = b[i] + scalar * c[i];
            }
        }
        black_box(a[0]);
    });

    let (med_ms, iqr_ms) = median_iqr(&mut samples);
    // Standard STREAM Triad counts 3 arrays (read b, read c, write a = 12 B/elem).
    // Hardware traffic with write-allocate (RFO) is 4 arrays (16 B/elem).
    let bytes_std = (3 * n * 4) as f64;
    let bytes_rfo = (4 * n * 4) as f64;
    let sec = med_ms / 1_000.0;
    let gbs_std = bytes_std / sec / 1e9;
    let gibs_std = bytes_std / sec / (1024.0 * 1024.0 * 1024.0);
    let gbs_rfo = bytes_rfo / sec / 1e9;
    let gibs_rfo = bytes_rfo / sec / (1024.0 * 1024.0 * 1024.0);
    println!(
        "STREAM,{},{},{},{:.4},{:.4},{:.2},{:.2},{:.2},{:.2}",
        name,
        if multi { worker_count() } else { 1 },
        n,
        med_ms,
        iqr_ms,
        gbs_std,
        gibs_std,
        gbs_rfo,
        gibs_rfo
    );
}

fn run_gemm_crate(a: &Tensor, b: &Tensor, c: &mut Tensor, par: Parallelism) {
    let (m, k) = a.dims_2d();
    let (_, n) = b.dims_2d();
    unsafe {
        gemm(
            m,
            n,
            k,
            c.as_mut_slice().as_mut_ptr(),
            1,
            n as isize,
            false,
            a.as_slice().as_ptr(),
            1,
            k as isize,
            b.as_slice().as_ptr(),
            1,
            n as isize,
            0.0_f32,
            1.0_f32,
            false,
            false,
            false,
            par,
        );
    }
}

fn main() {
    let threads = worker_count();
    println!("# STREAM_HEADER: kind,region,threads,n_elems,med_ms,iqr_ms,stream_gb_s,stream_gib_s,rfo_gb_s,rfo_gib_s");
    // L3-resident: 3 * 1M * 4 B = 12 MiB (fits inside 32 MiB L3)
    run_stream_triad("L3_12MiB", 1_048_576, false, 5, 40);
    run_stream_triad("L3_12MiB", 1_048_576, true, 5, 40);
    // DRAM-resident: 3 * 16M * 4 B = 192 MiB (6x larger than 32 MiB L3)
    run_stream_triad("DRAM_192MiB", 16_777_216, false, 5, 30);
    run_stream_triad("DRAM_192MiB", 16_777_216, true, 5, 30);

    println!("# MATMUL_HEADER: kind,impl,size,threads,med_ms,iqr_ms,gflops");
    for &sz in &[128usize, 512, 768, 1024] {
        let a = Tensor::random(&[sz, sz], 55);
        let b = Tensor::random(&[sz, sz], 66);
        let mut c_gemm = Tensor::zeros(&[sz, sz]);
        let flops = 2.0 * (sz as f64).powi(3);
        let iters = if sz <= 512 { 30 } else { 20 };

        // Validate `gemm` numerical parity against our AVX2+FMA matmul once
        run_gemm_crate(&a, &b, &mut c_gemm, Parallelism::None);
        let ref_c = avx2::matmul(&a, &b);
        let diff = c_gemm.max_abs_diff(&ref_c);
        assert!(diff < 1e-3, "gemm mismatch at {sz}: {diff}");

        let mut s = time_samples(3, iters, || {
            black_box(avx2::matmul_no_fma(black_box(&a), black_box(&b)));
        });
        let (med, iqr) = median_iqr(&mut s);
        println!("MATMUL,avx2_no_fma,{sz},1,{med:.4},{iqr:.4},{:.2}", flops / (med / 1e3) / 1e9);

        let mut s = time_samples(3, iters, || {
            black_box(avx2::matmul(black_box(&a), black_box(&b)));
        });
        let (med, iqr) = median_iqr(&mut s);
        println!("MATMUL,avx2_fma,{sz},1,{med:.4},{iqr:.4},{:.2}", flops / (med / 1e3) / 1e9);

        let mut s = time_samples(3, iters, || {
            run_gemm_crate(black_box(&a), black_box(&b), black_box(&mut c_gemm), Parallelism::None);
        });
        let (med, iqr) = median_iqr(&mut s);
        println!("MATMUL,gemm_crate_1t,{sz},1,{med:.4},{iqr:.4},{:.2}", flops / (med / 1e3) / 1e9);

        let mut s = time_samples(3, iters, || {
            black_box(multicore::avx2_mt::matmul_no_fma(black_box(&a), black_box(&b)));
        });
        let (med, iqr) = median_iqr(&mut s);
        println!("MATMUL,mc_avx2_no_fma,{sz},{threads},{med:.4},{iqr:.4},{:.2}", flops / (med / 1e3) / 1e9);

        let mut s = time_samples(3, iters, || {
            black_box(multicore::avx2_mt::matmul(black_box(&a), black_box(&b)));
        });
        let (med, iqr) = median_iqr(&mut s);
        println!("MATMUL,mc_avx2_fma,{sz},{threads},{med:.4},{iqr:.4},{:.2}", flops / (med / 1e3) / 1e9);

        let mut s = time_samples(3, iters, || {
            run_gemm_crate(black_box(&a), black_box(&b), black_box(&mut c_gemm), Parallelism::Rayon(threads));
        });
        let (med, iqr) = median_iqr(&mut s);
        println!("MATMUL,gemm_crate_mt,{sz},{threads},{med:.4},{iqr:.4},{:.2}", flops / (med / 1e3) / 1e9);
    }

    // 2048x2048 crossover (CPU 8T vs gemm 8T vs RTX 2070 GPU)
    if cuda_backend::is_available() {
        let dev = cuda_backend::device_name().unwrap_or_else(|| "CUDA".into());
        println!("# GPU_DEVICE: {dev}");
        for &sz in &[512usize, 768, 1024, 2048] {
            let a = Tensor::random(&[sz, sz], 55);
            let b = Tensor::random(&[sz, sz], 66);
            let flops = 2.0 * (sz as f64).powi(3);
            let iters = if sz <= 1024 { 25 } else { 15 };

            if sz == 2048 {
                let mut c_gemm = Tensor::zeros(&[sz, sz]);
                let mut s = time_samples(2, 10, || {
                    black_box(multicore::avx2_mt::matmul(black_box(&a), black_box(&b)));
                });
                let (med, iqr) = median_iqr(&mut s);
                println!("MATMUL,mc_avx2_fma,{sz},{threads},{med:.4},{iqr:.4},{:.2}", flops / (med / 1e3) / 1e9);

                let mut s = time_samples(2, 10, || {
                    run_gemm_crate(black_box(&a), black_box(&b), black_box(&mut c_gemm), Parallelism::Rayon(threads));
                });
                let (med, iqr) = median_iqr(&mut s);
                println!("MATMUL,gemm_crate_mt,{sz},{threads},{med:.4},{iqr:.4},{:.2}", flops / (med / 1e3) / 1e9);
            }

            let (mut k_ms, mut e2e_ms) = cuda_backend::bench_matmul_breakdown(&a, &b, 5, iters);
            let (k_med, k_iqr) = median_iqr(&mut k_ms);
            let (e_med, e_iqr) = median_iqr(&mut e2e_ms);
            println!("MATMUL,gpu_cuda_kernel,{sz},1,{k_med:.4},{k_iqr:.4},{:.2}", flops / (k_med / 1e3) / 1e9);
            println!("MATMUL,gpu_cuda_pcie_e2e,{sz},1,{e_med:.4},{e_iqr:.4},{:.2}", flops / (e_med / 1e3) / 1e9);
        }
    }
}
