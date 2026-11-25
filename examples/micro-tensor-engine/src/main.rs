use std::hint::black_box;
use std::time::Instant;

use micro_tensor_engine::kernels::{avx2, multicore, scalar, worker_count};
#[cfg(feature = "engine-mv")]
use micro_tensor_engine::MultiversionEngine;
use micro_tensor_engine::platform_engine::ops::{dispatch_probe, dispatch_probe_second};
use micro_tensor_engine::static_engine::{
    AdHocRuntimeEngine, StaticCfgEngine, direct_axpy, direct_dot, direct_matmul, direct_matvec,
    direct_relu, static_dispatch_probe,
};
use micro_tensor_engine::{AdHocIdiomaticEngine, Tensor, TensorOps, cuda_backend, dot_reference_f64};

#[derive(Clone, Copy)]
struct BenchPreset {
    name: &'static str,
    vec_len: usize,
    mv_m: usize,
    mv_k: usize,
    mm_m: usize,
    mm_k: usize,
    mm_n: usize,
}

impl BenchPreset {
    fn from_name(name: &str) -> Self {
        match name {
            "small" => Self {
                name: "small",
                vec_len: 262_144,
                mv_m: 1024,
                mv_k: 1024,
                mm_m: 128,
                mm_k: 128,
                mm_n: 128,
            },
            "large" => Self {
                name: "large",
                vec_len: 8_388_608,
                mv_m: 4096,
                mv_k: 4096,
                mm_m: 768,
                mm_k: 768,
                mm_n: 768,
            },
            _ => Self {
                name: "medium",
                vec_len: 4_194_304,
                mv_m: 2048,
                mv_k: 2048,
                mm_m: 512,
                mm_k: 512,
                mm_n: 512,
            },
        }
    }
}

struct CliConfig {
    preset: BenchPreset,
    warmup: usize,
    iters: usize,
    overhead_calls: usize,
    csv: bool,
    probe_only: bool,
    relu_diag: bool,
    tost_kernels: bool,
}

fn parse_args() -> CliConfig {
    let mut size = "medium".to_string();
    let mut warmup = 5usize;
    let mut iters = 50usize;
    let mut overhead_calls = 1_000_000usize;
    let mut csv = false;
    let mut probe_only = false;
    let mut relu_diag = false;
    let mut tost_kernels = false;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--size" => {
                if let Some(v) = args.next() {
                    size = v;
                }
            }
            "--warmup" => {
                if let Some(v) = args.next() {
                    warmup = v.parse().expect("--warmup expects an integer");
                }
            }
            "--iters" => {
                if let Some(v) = args.next() {
                    iters = v.parse().expect("--iters expects an integer");
                }
            }
            "--overhead-calls" => {
                if let Some(v) = args.next() {
                    overhead_calls = v.parse().expect("--overhead-calls expects an integer");
                }
            }
            "--csv" => csv = true,
            "--probe-only" => probe_only = true,
            "--relu-diag" => relu_diag = true,
            "--tost-kernels" => tost_kernels = true,
            "--help" | "-h" => {
                println!(
                    "Usage: micro-tensor-engine [--size small|medium|large] [--warmup N] [--iters N] [--overhead-calls N] [--csv] [--probe-only] [--relu-diag] [--tost-kernels]"
                );
                std::process::exit(0);
            }
            other => {
                eprintln!("Unknown argument: {other}");
                std::process::exit(2);
            }
        }
    }

    CliConfig {
        preset: BenchPreset::from_name(&size),
        warmup,
        iters: iters.max(1),
        overhead_calls: overhead_calls.max(1),
        csv,
        probe_only,
        relu_diag,
        tost_kernels,
    }
}

#[derive(Clone, Copy)]
struct TimingStats {
    median_ms: f64,
    iqr_ms: f64,
    p05_ms: f64,
    p95_ms: f64,
    mean_ms: f64,
    stddev_ms: f64,
}

fn percentile_sorted(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = p * ((sorted.len() - 1) as f64);
    let lo = idx.floor() as usize;
    let hi = idx.ceil() as usize;
    let frac = idx - (lo as f64);
    sorted[lo] * (1.0 - frac) + sorted[hi] * frac
}

/// Measures `iters` samples, automatically batching inner repetitions if a single call
/// takes < 0.5 ms so sub-millisecond operations avoid timer quantization artifacts.
fn measure_stats_ms<F: FnMut()>(warmup: usize, iters: usize, mut f: F) -> TimingStats {
    for _ in 0..warmup {
        f();
    }
    let probe_t0 = Instant::now();
    f();
    let probe_ms = probe_t0.elapsed().as_secs_f64() * 1_000.0;
    let inner_reps: usize = if probe_ms < 0.05 {
        64
    } else if probe_ms < 0.25 {
        16
    } else if probe_ms < 0.5 {
        4
    } else {
        1
    };

    let mut samples = Vec::with_capacity(iters);
    for _ in 0..iters {
        let t0 = Instant::now();
        for _ in 0..inner_reps {
            f();
        }
        samples.push(t0.elapsed().as_secs_f64() * 1_000.0 / (inner_reps as f64));
    }

    let mean_ms = samples.iter().sum::<f64>() / (samples.len() as f64);
    let var = if samples.len() > 1 {
        samples
            .iter()
            .map(|x| {
                let d = x - mean_ms;
                d * d
            })
            .sum::<f64>()
            / ((samples.len() - 1) as f64)
    } else {
        0.0
    };
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median_ms = percentile_sorted(&samples, 0.50);
    let q1 = percentile_sorted(&samples, 0.25);
    let q3 = percentile_sorted(&samples, 0.75);
    TimingStats {
        median_ms,
        iqr_ms: q3 - q1,
        p05_ms: percentile_sorted(&samples, 0.05),
        p95_ms: percentile_sorted(&samples, 0.95),
        mean_ms,
        stddev_ms: var.sqrt(),
    }
}

fn check_host_can_run(backend: &str) {
    if backend == "gpu_cuda" {
        if !cuda_backend::is_available() {
            eprintln!("ERROR: Platform.toml selected `gpu_cuda`, but CUDA runtime is unavailable.");
            std::process::exit(1);
        }
        return;
    }
    #[cfg(target_arch = "x86_64")]
    {
        if matches!(backend, "avx512" | "multicore_avx512") && !is_x86_feature_detected!("avx512f")
        {
            eprintln!(
                "ERROR: Platform.toml selected `{backend}`, but this CPU does not support AVX-512F."
            );
            std::process::exit(1);
        }
        if matches!(backend, "avx2" | "multicore_avx2")
            && !(is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma"))
        {
            eprintln!(
                "ERROR: Platform.toml selected `{backend}`, but this CPU lacks AVX2+FMA."
            );
            std::process::exit(1);
        }
    }
}

struct OpResult {
    op: &'static str,
    unit: &'static str,
    work: f64,
    is_bytes: bool,
    static_cfg: TimingStats,
    static_direct: TimingStats,
    adhoc: TimingStats,
    platform_aware: TimingStats,
    max_abs_diff: f64,
    max_rel_err: f64,
}

fn throughput(work: f64, ms: f64, is_bytes: bool) -> f64 {
    let sec = ms / 1_000.0;
    if is_bytes {
        // Binary GiB/s consistent with MiB tensor footprints (2^20 / 2^30)
        work / sec / (1024.0 * 1024.0 * 1024.0)
    } else {
        // Decimal GFLOP/s (10^9 FLOP/s)
        work / sec / 1e9
    }
}

fn measure_probe<F: Fn(u64) -> u64>(calls: usize, f: F) -> f64 {
    let t0 = Instant::now();
    let mut acc = 0u64;
    for i in 0..calls as u64 {
        acc = f(black_box(acc ^ i));
    }
    black_box(acc);
    t0.elapsed().as_secs_f64() * 1e9 / (calls as f64)
}

/// Diagnostic isolating three mechanisms in `relu`:
///   1. `alloc`        : fresh `Tensor::zeros` per call (triggers lazy mmap + minor page faults under pool)
///   2. `alloc_touched`: fresh allocation pre-touched serially before kernel (separates allocation from parallel page faults)
///   3. `prealloc`     : reused pre-allocated output buffer (`relu_into`, zero page faults)
/// Memory traffic accounts for x86 write-allocate (RFO): read x + RFO out + write out = 3 * N * 4 B.
fn run_relu_diag(cfg: &CliConfig) {
    let p = cfg.preset;
    let v1 = Tensor::random(&[p.vec_len], 11);
    let mut out = Tensor::zeros(&[p.vec_len]);
    for slot in out.as_mut_slice() {
        *slot = 0.0;
    }
    // Effective memory traffic with x86 write-allocate (RFO) = 3 * N * 4 bytes
    let work_rfo = (3 * p.vec_len * std::mem::size_of::<f32>()) as f64;
    let threads = worker_count();

    println!("preset,threads,mode,variant,med_ms,iqr_ms,rfo_gib_s,rfo_gb_s");
    let cases: Vec<(&str, &str, TimingStats)> = vec![
        ("alloc", "scalar", measure_stats_ms(cfg.warmup, cfg.iters, || { black_box(scalar::relu(black_box(&v1))); })),
        ("alloc", "avx2", measure_stats_ms(cfg.warmup, cfg.iters, || { black_box(avx2::relu(black_box(&v1))); })),
        ("alloc", "multicore_scalar", measure_stats_ms(cfg.warmup, cfg.iters, || { black_box(multicore::scalar_mt::relu(black_box(&v1))); })),
        ("alloc", "multicore_avx2", measure_stats_ms(cfg.warmup, cfg.iters, || { black_box(multicore::avx2_mt::relu(black_box(&v1))); })),
        ("alloc_touched", "scalar", measure_stats_ms(cfg.warmup, cfg.iters, || {
            let mut buf = Tensor::zeros(v1.shape());
            for s in buf.as_mut_slice().iter_mut().step_by(1024) { *s = 0.0; }
            scalar::relu_into(black_box(&v1), black_box(&mut buf));
            black_box(buf);
        })),
        ("alloc_touched", "multicore_avx2", measure_stats_ms(cfg.warmup, cfg.iters, || {
            let mut buf = Tensor::zeros(v1.shape());
            for s in buf.as_mut_slice().iter_mut().step_by(1024) { *s = 0.0; }
            multicore::avx2_mt::relu_into(black_box(&v1), black_box(&mut buf));
            black_box(buf);
        })),
        ("prealloc", "scalar", measure_stats_ms(cfg.warmup, cfg.iters, || { scalar::relu_into(black_box(&v1), black_box(&mut out)); })),
        ("prealloc", "avx2", measure_stats_ms(cfg.warmup, cfg.iters, || { avx2::relu_into(black_box(&v1), black_box(&mut out)); })),
        ("prealloc", "multicore_scalar", measure_stats_ms(cfg.warmup, cfg.iters, || { multicore::scalar_mt::relu_into(black_box(&v1), black_box(&mut out)); })),
        ("prealloc", "multicore_avx2", measure_stats_ms(cfg.warmup, cfg.iters, || { multicore::avx2_mt::relu_into(black_box(&v1), black_box(&mut out)); })),
    ];
    for (mode, variant, st) in cases {
        let gib_s = throughput(work_rfo, st.median_ms, true);
        let gb_s = work_rfo / (st.median_ms / 1_000.0) / 1e9;
        println!(
            "{},{},{},{},{:.4},{:.4},{:.2},{:.2}",
            p.name, threads, mode, variant, st.median_ms, st.iqr_ms, gib_s, gb_s
        );
    }
}

/// Collects raw interleaved per-iteration samples on ISOLATED PURE KERNELS (no xorshift64* setup
/// or process startup inside the timer) for rigorous non-diluted TOST equivalence testing.
fn run_tost_kernels(cfg: &CliConfig) {
    let p = cfg.preset;
    let probe = Tensor::zeros(&[1]);
    let pa_backend = probe.active_backend();
    let static_backend = StaticCfgEngine::active_backend();
    check_host_can_run(pa_backend);

    let v1 = Tensor::random(&[p.vec_len], 11);
    let v2 = Tensor::random(&[p.vec_len], 22);
    let a_mv = Tensor::random(&[p.mv_m, p.mv_k], 33);
    let x_mv = Tensor::random(&[p.mv_k], 44);
    let a_mm = Tensor::random(&[p.mm_m, p.mm_k], 55);
    let b_mm = Tensor::random(&[p.mm_k, p.mm_n], 66);

    // Warm up every engine first (including LazyLock/OnceLock/ParallelPool)
    let mut y_pa = v2.clone();
    let mut y_st = v2.clone();
    let mut y_ah = v2.clone();
    let mut y_ai = v2.clone();
    #[cfg(feature = "engine-mv")]
    let mut y_mv = v2.clone();
    for _ in 0..cfg.warmup {
        black_box(v1.dot(&v2));
        black_box(StaticCfgEngine::dot(&v1, &v2));
        black_box(AdHocRuntimeEngine::dot(&v1, &v2));
        black_box(AdHocIdiomaticEngine::dot(&v1, &v2));
        #[cfg(feature = "engine-mv")]
        black_box(MultiversionEngine::dot(&v1, &v2));
        y_pa.axpy(0.25, &v1);
        StaticCfgEngine::axpy(0.25, &v1, &mut y_st);
        AdHocRuntimeEngine::axpy(0.25, &v1, &mut y_ah);
        AdHocIdiomaticEngine::axpy(0.25, &v1, &mut y_ai);
        #[cfg(feature = "engine-mv")]
        MultiversionEngine::axpy(0.25, &v1, &mut y_mv);
        black_box(a_mm.matmul(&b_mm));
        black_box(StaticCfgEngine::matmul(&a_mm, &b_mm));
        black_box(AdHocRuntimeEngine::matmul(&a_mm, &b_mm));
        black_box(AdHocIdiomaticEngine::matmul(&a_mm, &b_mm));
        #[cfg(feature = "engine-mv")]
        black_box(MultiversionEngine::matmul(&a_mm, &b_mm));
    }

    println!("op,iter,pa_backend,static_backend,pa_ms,static_cfg_ms,direct_ms,adhoc_ms,adhoc_idiom_ms,mv_ms");

    // Latin-square rotated engine order per operation eliminates L3 cache-order and DVFS drift bias
    // (Hoefler & Belli, SC'15; Mytkowicz et al., ASPLOS'09).
    let ops = ["dot", "axpy", "relu", "matvec", "matmul"];
    for &op in &ops {
        // Dedicated warmup for this specific operation so working set is identically primed
        for _ in 0..cfg.warmup {
            match op {
                "dot" => { black_box(v1.dot(&v2)); }
                "axpy" => { y_pa.axpy(0.25, &v1); }
                "relu" => { black_box(v1.relu()); }
                "matvec" => { black_box(a_mv.matvec(&x_mv)); }
                "matmul" => { black_box(a_mm.matmul(&b_mm)); }
                _ => {}
            }
        }
        for it in 0..cfg.iters {
            let mut times = [0.0_f64; 6];
            for step in 0..6 {
                let eng = (it + step) % 6;
                let t = Instant::now();
                match (op, eng) {
                    ("dot", 0) => { black_box(black_box(&v1).dot(black_box(&v2))); }
                    ("dot", 1) => { black_box(StaticCfgEngine::dot(black_box(&v1), black_box(&v2))); }
                    ("dot", 2) => { black_box(direct_dot(pa_backend, black_box(&v1), black_box(&v2))); }
                    ("dot", 3) => { black_box(AdHocRuntimeEngine::dot(black_box(&v1), black_box(&v2))); }
                    ("dot", 4) => { black_box(AdHocIdiomaticEngine::dot(black_box(&v1), black_box(&v2))); }
                    ("dot", 5) => {
                        #[cfg(feature = "engine-mv")]
                        { black_box(MultiversionEngine::dot(black_box(&v1), black_box(&v2))); }
                    }
                    ("axpy", 0) => { black_box(&mut y_pa).axpy(0.25, black_box(&v1)); }
                    ("axpy", 1) => { StaticCfgEngine::axpy(0.25, black_box(&v1), black_box(&mut y_st)); }
                    ("axpy", 2) => { direct_axpy(pa_backend, 0.25, black_box(&v1), black_box(&mut y_pa)); }
                    ("axpy", 3) => { AdHocRuntimeEngine::axpy(0.25, black_box(&v1), black_box(&mut y_ah)); }
                    ("axpy", 4) => { AdHocIdiomaticEngine::axpy(0.25, black_box(&v1), black_box(&mut y_ai)); }
                    ("axpy", 5) => {
                        #[cfg(feature = "engine-mv")]
                        { MultiversionEngine::axpy(0.25, black_box(&v1), black_box(&mut y_mv)); }
                    }
                    ("relu", 0) => { black_box(black_box(&v1).relu()); }
                    ("relu", 1) => { black_box(StaticCfgEngine::relu(black_box(&v1))); }
                    ("relu", 2) => { black_box(direct_relu(pa_backend, black_box(&v1))); }
                    ("relu", 3) => { black_box(AdHocRuntimeEngine::relu(black_box(&v1))); }
                    ("relu", 4) => { black_box(AdHocIdiomaticEngine::relu(black_box(&v1))); }
                    ("relu", 5) => {
                        #[cfg(feature = "engine-mv")]
                        { black_box(MultiversionEngine::relu(black_box(&v1))); }
                    }
                    ("matvec", 0) => { black_box(black_box(&a_mv).matvec(black_box(&x_mv))); }
                    ("matvec", 1) => { black_box(StaticCfgEngine::matvec(black_box(&a_mv), black_box(&x_mv))); }
                    ("matvec", 2) => { black_box(direct_matvec(pa_backend, black_box(&a_mv), black_box(&x_mv))); }
                    ("matvec", 3) => { black_box(AdHocRuntimeEngine::matvec(black_box(&a_mv), black_box(&x_mv))); }
                    ("matvec", 4) => { black_box(AdHocIdiomaticEngine::matvec(black_box(&a_mv), black_box(&x_mv))); }
                    ("matvec", 5) => {
                        #[cfg(feature = "engine-mv")]
                        { black_box(MultiversionEngine::matvec(black_box(&a_mv), black_box(&x_mv))); }
                    }
                    ("matmul", 0) => { black_box(black_box(&a_mm).matmul(black_box(&b_mm))); }
                    ("matmul", 1) => { black_box(StaticCfgEngine::matmul(black_box(&a_mm), black_box(&b_mm))); }
                    ("matmul", 2) => { black_box(direct_matmul(pa_backend, black_box(&a_mm), black_box(&b_mm))); }
                    ("matmul", 3) => { black_box(AdHocRuntimeEngine::matmul(black_box(&a_mm), black_box(&b_mm))); }
                    ("matmul", 4) => { black_box(AdHocIdiomaticEngine::matmul(black_box(&a_mm), black_box(&b_mm))); }
                    ("matmul", 5) => {
                        #[cfg(feature = "engine-mv")]
                        { black_box(MultiversionEngine::matmul(black_box(&a_mm), black_box(&b_mm))); }
                    }
                    _ => {}
                }
                times[eng] = t.elapsed().as_secs_f64() * 1e3;
            }
            println!(
                "{op},{it},{pa_backend},{static_backend},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6}",
                times[0], times[1], times[2], times[3], times[4], times[5]
            );
        }
    }
}

fn main() {
    let cfg = parse_args();
    if cfg.relu_diag {
        run_relu_diag(&cfg);
        return;
    }
    if cfg.tost_kernels {
        run_tost_kernels(&cfg);
        return;
    }
    let p = cfg.preset;

    // 1a. First k-function call in process: global Platform.toml load (Once) + resolve + LazyLock
    let t_first = Instant::now();
    let _ = black_box(dispatch_probe(black_box(0)));
    let first_call_us = t_first.elapsed().as_secs_f64() * 1_000_000.0;

    // 1b. Second independent k-function first call: resolve + LazyLock only (TOML already cached)
    let t_second = Instant::now();
    let _ = black_box(dispatch_probe_second(black_box(0)));
    let second_kfunc_us = t_second.elapsed().as_secs_f64() * 1_000_000.0;

    // 1c. First call of multiversion (CPUID + atomic function-pointer cache), if enabled
    #[cfg(feature = "engine-mv")]
    let mv_first_call_us = {
        let t_mv = Instant::now();
        let _ = black_box(MultiversionEngine::dispatch_probe(black_box(0)));
        t_mv.elapsed().as_secs_f64() * 1_000_000.0
    };
    #[cfg(not(feature = "engine-mv"))]
    let mv_first_call_us = 0.0_f64;

    // 1d. First call of idiomatic ad-hoc (`OnceLock<&'static dyn KernelBackend>`)
    let t_ai = Instant::now();
    let _ = black_box(AdHocIdiomaticEngine::dispatch_probe(black_box(0)));
    let ai_first_call_us = t_ai.elapsed().as_secs_f64() * 1_000_000.0;

    // 2. Measure amortized per-call dispatch overhead
    let static_probe_ns = measure_probe(cfg.overhead_calls, static_dispatch_probe);
    let pa_probe_ns = measure_probe(cfg.overhead_calls, dispatch_probe);
    let adhoc_probe_ns = measure_probe(cfg.overhead_calls, AdHocRuntimeEngine::dispatch_probe);
    let adhoc_idiom_probe_ns = measure_probe(cfg.overhead_calls, AdHocIdiomaticEngine::dispatch_probe);
    #[cfg(feature = "engine-mv")]
    let mv_probe_ns = measure_probe(cfg.overhead_calls, MultiversionEngine::dispatch_probe);
    #[cfg(not(feature = "engine-mv"))]
    let mv_probe_ns = 0.0_f64;

    if cfg.probe_only {
        println!(
            "{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3}",
            first_call_us,
            second_kfunc_us,
            mv_first_call_us,
            ai_first_call_us,
            static_probe_ns,
            pa_probe_ns,
            adhoc_probe_ns,
            adhoc_idiom_probe_ns,
            mv_probe_ns
        );
        return;
    }

    let probe_tensor = Tensor::zeros(&[1]);
    let pa_backend = probe_tensor.active_backend();
    let static_cfg_backend = StaticCfgEngine::active_backend();
    let adhoc_backend = AdHocRuntimeEngine::active_backend();

    check_host_can_run(pa_backend);

    // Allocate deterministic test tensors
    let v1 = Tensor::random(&[p.vec_len], 11);
    let v2 = Tensor::random(&[p.vec_len], 22);
    let a_mv = Tensor::random(&[p.mv_m, p.mv_k], 33);
    let x_mv = Tensor::random(&[p.mv_k], 44);
    let a_mm = Tensor::random(&[p.mm_m, p.mm_k], 55);
    let b_mm = Tensor::random(&[p.mm_k, p.mm_n], 66);

    // Reference outputs for numerical correctness validation (f64 exact reduction for dot)
    let ref_dot_f64 = dot_reference_f64(&v1, &v2);
    let mut ref_axpy = v2.clone();
    scalar::axpy(0.25, &v1, &mut ref_axpy);
    let ref_relu = scalar::relu(&v1);
    let ref_mv = scalar::matvec(&a_mv, &x_mv);
    let ref_mm = scalar::matmul(&a_mm, &b_mm);

    let max_abs_tensor = |t: &Tensor| -> f64 {
        t.as_slice().iter().map(|v| (*v as f64).abs()).fold(0.0_f64, f64::max).max(1e-12)
    };

    let mut results = Vec::new();

    // --- dot (read a + read b = 2 * N * 4 B) ---
    {
        let got_dot = v1.dot(&v2) as f64;
        let diff = (got_dot - ref_dot_f64).abs();
        let rel = diff / ref_dot_f64.abs().max(1e-12);
        let static_cfg = measure_stats_ms(cfg.warmup, cfg.iters, || {
            black_box(StaticCfgEngine::dot(black_box(&v1), black_box(&v2)));
        });
        let static_direct = measure_stats_ms(cfg.warmup, cfg.iters, || {
            black_box(direct_dot(pa_backend, black_box(&v1), black_box(&v2)));
        });
        let adhoc = measure_stats_ms(cfg.warmup, cfg.iters, || {
            black_box(AdHocRuntimeEngine::dot(black_box(&v1), black_box(&v2)));
        });
        let platform_aware = measure_stats_ms(cfg.warmup, cfg.iters, || {
            black_box(black_box(&v1).dot(black_box(&v2)));
        });
        results.push(OpResult {
            op: "dot",
            unit: "GiB/s",
            work: (2 * p.vec_len * std::mem::size_of::<f32>()) as f64,
            is_bytes: true,
            static_cfg,
            static_direct,
            adhoc,
            platform_aware,
            max_abs_diff: diff,
            max_rel_err: rel,
        });
    }

    // --- axpy (read x + read y + write y = 3 * N * 4 B) ---
    {
        let mut y_check = v2.clone();
        y_check.axpy(0.25, &v1);
        let diff = y_check.max_abs_diff(&ref_axpy) as f64;
        let rel = diff / max_abs_tensor(&ref_axpy);

        let mut y_buf = v2.clone();
        let static_cfg = measure_stats_ms(cfg.warmup, cfg.iters, || {
            StaticCfgEngine::axpy(0.25, black_box(&v1), black_box(&mut y_buf));
        });
        let mut y_buf = v2.clone();
        let static_direct = measure_stats_ms(cfg.warmup, cfg.iters, || {
            direct_axpy(pa_backend, 0.25, black_box(&v1), black_box(&mut y_buf));
        });
        let mut y_buf = v2.clone();
        let adhoc = measure_stats_ms(cfg.warmup, cfg.iters, || {
            AdHocRuntimeEngine::axpy(0.25, black_box(&v1), black_box(&mut y_buf));
        });
        let mut y_buf = v2.clone();
        let platform_aware = measure_stats_ms(cfg.warmup, cfg.iters, || {
            black_box(&mut y_buf).axpy(0.25, black_box(&v1));
        });
        results.push(OpResult {
            op: "axpy",
            unit: "GiB/s",
            work: (3 * p.vec_len * std::mem::size_of::<f32>()) as f64,
            is_bytes: true,
            static_cfg,
            static_direct,
            adhoc,
            platform_aware,
            max_abs_diff: diff,
            max_rel_err: rel,
        });
    }

    // --- relu (read x + write-allocate RFO out + write out = 3 * N * 4 B) ---
    {
        let diff = v1.relu().max_abs_diff(&ref_relu) as f64;
        let rel = diff / max_abs_tensor(&ref_relu);
        let static_cfg = measure_stats_ms(cfg.warmup, cfg.iters, || {
            black_box(StaticCfgEngine::relu(black_box(&v1)));
        });
        let static_direct = measure_stats_ms(cfg.warmup, cfg.iters, || {
            black_box(direct_relu(pa_backend, black_box(&v1)));
        });
        let adhoc = measure_stats_ms(cfg.warmup, cfg.iters, || {
            black_box(AdHocRuntimeEngine::relu(black_box(&v1)));
        });
        let platform_aware = measure_stats_ms(cfg.warmup, cfg.iters, || {
            black_box(black_box(&v1).relu());
        });
        results.push(OpResult {
            op: "relu",
            unit: "GiB/s",
            work: (3 * p.vec_len * std::mem::size_of::<f32>()) as f64,
            is_bytes: true,
            static_cfg,
            static_direct,
            adhoc,
            platform_aware,
            max_abs_diff: diff,
            max_rel_err: rel,
        });
    }

    // --- matvec ---
    {
        let diff = a_mv.matvec(&x_mv).max_abs_diff(&ref_mv) as f64;
        let rel = diff / max_abs_tensor(&ref_mv);
        let static_cfg = measure_stats_ms(cfg.warmup, cfg.iters, || {
            black_box(StaticCfgEngine::matvec(black_box(&a_mv), black_box(&x_mv)));
        });
        let static_direct = measure_stats_ms(cfg.warmup, cfg.iters, || {
            black_box(direct_matvec(pa_backend, black_box(&a_mv), black_box(&x_mv)));
        });
        let adhoc = measure_stats_ms(cfg.warmup, cfg.iters, || {
            black_box(AdHocRuntimeEngine::matvec(black_box(&a_mv), black_box(&x_mv)));
        });
        let platform_aware = measure_stats_ms(cfg.warmup, cfg.iters, || {
            black_box(black_box(&a_mv).matvec(black_box(&x_mv)));
        });
        results.push(OpResult {
            op: "matvec",
            unit: "GFLOP/s",
            work: (2 * p.mv_m * p.mv_k) as f64,
            is_bytes: false,
            static_cfg,
            static_direct,
            adhoc,
            platform_aware,
            max_abs_diff: diff,
            max_rel_err: rel,
        });
    }

    // --- matmul ---
    {
        let diff = a_mm.matmul(&b_mm).max_abs_diff(&ref_mm) as f64;
        let rel = diff / max_abs_tensor(&ref_mm);
        let static_cfg = measure_stats_ms(cfg.warmup, cfg.iters, || {
            black_box(StaticCfgEngine::matmul(black_box(&a_mm), black_box(&b_mm)));
        });
        let static_direct = measure_stats_ms(cfg.warmup, cfg.iters, || {
            black_box(direct_matmul(pa_backend, black_box(&a_mm), black_box(&b_mm)));
        });
        let adhoc = measure_stats_ms(cfg.warmup, cfg.iters, || {
            black_box(AdHocRuntimeEngine::matmul(black_box(&a_mm), black_box(&b_mm)));
        });
        let platform_aware = measure_stats_ms(cfg.warmup, cfg.iters, || {
            black_box(black_box(&a_mm).matmul(black_box(&b_mm)));
        });
        results.push(OpResult {
            op: "matmul",
            unit: "GFLOP/s",
            work: (2 * p.mm_m * p.mm_k * p.mm_n) as f64,
            is_bytes: false,
            static_cfg,
            static_direct,
            adhoc,
            platform_aware,
            max_abs_diff: diff,
            max_rel_err: rel,
        });
    }

    if cfg.csv {
        println!(
            "preset,pa_backend,static_cfg_backend,adhoc_backend,op,static_cfg_med_ms,static_cfg_iqr_ms,direct_med_ms,direct_iqr_ms,adhoc_med_ms,adhoc_iqr_ms,pa_med_ms,pa_iqr_ms,pa_p05_ms,pa_p95_ms,pa_mean_ms,pa_std_ms,speedup_vs_static_cfg,overhead_vs_direct_pct,pa_throughput,unit,max_abs_diff,max_rel_err"
        );
        for r in &results {
            let speedup_cfg = r.static_cfg.median_ms / r.platform_aware.median_ms;
            let overhead_pct = ((r.platform_aware.median_ms - r.static_direct.median_ms)
                / r.static_direct.median_ms)
                * 100.0;
            println!(
                "{},{},{},{},{},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.2},{:+.2},{:.2},{},{:.2e},{:.2e}",
                p.name,
                pa_backend,
                static_cfg_backend,
                adhoc_backend,
                r.op,
                r.static_cfg.median_ms,
                r.static_cfg.iqr_ms,
                r.static_direct.median_ms,
                r.static_direct.iqr_ms,
                r.adhoc.median_ms,
                r.adhoc.iqr_ms,
                r.platform_aware.median_ms,
                r.platform_aware.iqr_ms,
                r.platform_aware.p05_ms,
                r.platform_aware.p95_ms,
                r.platform_aware.mean_ms,
                r.platform_aware.stddev_ms,
                speedup_cfg,
                overhead_pct,
                throughput(r.work, r.platform_aware.median_ms, r.is_bytes),
                r.unit,
                r.max_abs_diff,
                r.max_rel_err
            );
        }
        return;
    }

    let profile_src = std::env::var("PLATFORM_DESCRIPTION").unwrap_or_else(|_| "./Platform.toml".into());
    println!("==========================================================================================");
    println!(" Micro Tensor Engine Benchmark — platform-aware vs. Baselines (AVX2+FMA & RTX 2070 CUDA)");
    println!("==========================================================================================");
    println!(
        " Preset          : {} (vec={}, matvec={}x{}, matmul={}x{}x{})",
        p.name, p.vec_len, p.mv_m, p.mv_k, p.mm_m, p.mm_k, p.mm_n
    );
    println!(" Iterations      : {} warmup / {} timed (median/IQR), threads={}", cfg.warmup, cfg.iters, worker_count());
    println!(" Platform.toml   : {}", profile_src);
    println!(
        " Active backends : platform-aware=`{}` | static-cfg=`{}` | ad-hoc=`{}`",
        pa_backend, static_cfg_backend, adhoc_backend
    );
    println!("------------------------------------------------------------------------------------------");
    println!(
        " Dispatch (1st)  : pa_1st={:.1} us | pa_2nd={:.1} us | mv={:.1} us | adhoc_idiom={:.1} us",
        first_call_us, second_kfunc_us, mv_first_call_us, ai_first_call_us
    );
    println!(
        " Dispatch (amort): direct={:.2} ns | mv={:.2} ns | ai={:.2} ns | adhoc={:.2} ns | pa={:.2} ns",
        static_probe_ns, mv_probe_ns, adhoc_idiom_probe_ns, adhoc_probe_ns, pa_probe_ns
    );
    println!("------------------------------------------------------------------------------------------");
    for r in &results {
        let speedup_cfg = r.static_cfg.median_ms / r.platform_aware.median_ms;
        let rate = throughput(r.work, r.platform_aware.median_ms, r.is_bytes);
        println!(
            " {:<8} | cfg={:>7.3} (iqr {:>5.3}) | pa={:>7.3} (iqr {:>5.3}) | {:>5.2}x | {:>6.1} {:<7} | abs={:.1e} rel={:.1e}",
            r.op,
            r.static_cfg.median_ms,
            r.static_cfg.iqr_ms,
            r.platform_aware.median_ms,
            r.platform_aware.iqr_ms,
            speedup_cfg,
            rate,
            r.unit,
            r.max_abs_diff,
            r.max_rel_err
        );
    }
    println!("==========================================================================================");
}
