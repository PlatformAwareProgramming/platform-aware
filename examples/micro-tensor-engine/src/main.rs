use std::hint::black_box;
use std::time::Instant;

use micro_tensor_engine::kernels::{scalar, worker_count};
use micro_tensor_engine::platform_engine::ops::dispatch_probe;
use micro_tensor_engine::static_engine::{
    AdHocRuntimeEngine, StaticCfgEngine, direct_axpy, direct_dot, direct_matmul, direct_matvec,
    direct_relu, static_dispatch_probe,
};
use micro_tensor_engine::{Tensor, TensorOps};

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
}

fn parse_args() -> CliConfig {
    let mut size = "medium".to_string();
    let mut warmup = 3usize;
    let mut iters = 10usize;
    let mut overhead_calls = 1_000_000usize;
    let mut csv = false;

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
            "--help" | "-h" => {
                println!(
                    "Usage: micro-tensor-engine [--size small|medium|large] [--warmup N] [--iters N] [--overhead-calls N] [--csv]"
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
    }
}

fn measure_median_ms<F: FnMut()>(warmup: usize, iters: usize, mut f: F) -> f64 {
    for _ in 0..warmup {
        f();
    }
    let mut samples = Vec::with_capacity(iters);
    for _ in 0..iters {
        let t0 = Instant::now();
        f();
        samples.push(t0.elapsed().as_secs_f64() * 1_000.0);
    }
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    samples[samples.len() / 2]
}

fn check_host_can_run(backend: &str) {
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
                "ERROR: Platform.toml selected `{backend}`, but this CPU does not support AVX2+FMA."
            );
            std::process::exit(1);
        }
    }
}

struct OpResult {
    op: &'static str,
    unit: &'static str,
    work: f64,
    static_cfg_ms: f64,
    static_direct_ms: f64,
    adhoc_ms: f64,
    platform_aware_ms: f64,
    max_abs_diff: f32,
}

fn throughput(work: f64, ms: f64) -> f64 {
    work / (ms / 1_000.0) / 1e9
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

fn main() {
    let cfg = parse_args();
    let p = cfg.preset;

    // 1. Measure first-call resolution latency before any other k-function is invoked
    let t_first = Instant::now();
    let _ = black_box(dispatch_probe(black_box(0)));
    let first_call_us = t_first.elapsed().as_secs_f64() * 1_000_000.0;

    // 2. Measure amortized per-call dispatch overhead (Section 5.3)
    let static_probe_ns = measure_probe(cfg.overhead_calls, static_dispatch_probe);
    let pa_probe_ns = measure_probe(cfg.overhead_calls, dispatch_probe);
    let adhoc_probe_ns = measure_probe(cfg.overhead_calls, AdHocRuntimeEngine::dispatch_probe);

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

    // Scalar reference outputs for numerical correctness validation
    let ref_dot = scalar::dot(&v1, &v2);
    let mut ref_axpy = v2.clone();
    scalar::axpy(0.25, &v1, &mut ref_axpy);
    let ref_relu = scalar::relu(&v1);
    let ref_mv = scalar::matvec(&a_mv, &x_mv);
    let ref_mm = scalar::matmul(&a_mm, &b_mm);

    let mut results = Vec::new();

    // --- dot ---
    {
        let diff = (v1.dot(&v2) - ref_dot).abs();
        let static_cfg_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            black_box(StaticCfgEngine::dot(black_box(&v1), black_box(&v2)));
        });
        let static_direct_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            black_box(direct_dot(pa_backend, black_box(&v1), black_box(&v2)));
        });
        let adhoc_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            black_box(AdHocRuntimeEngine::dot(black_box(&v1), black_box(&v2)));
        });
        let platform_aware_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            black_box(black_box(&v1).dot(black_box(&v2)));
        });
        results.push(OpResult {
            op: "dot",
            unit: "GB/s",
            work: (2 * p.vec_len * std::mem::size_of::<f32>()) as f64,
            static_cfg_ms,
            static_direct_ms,
            adhoc_ms,
            platform_aware_ms,
            max_abs_diff: diff,
        });
    }

    // --- axpy ---
    {
        let mut y_check = v2.clone();
        y_check.axpy(0.25, &v1);
        let diff = y_check.max_abs_diff(&ref_axpy);

        let mut y_buf = v2.clone();
        let static_cfg_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            StaticCfgEngine::axpy(0.25, black_box(&v1), black_box(&mut y_buf));
        });
        let mut y_buf = v2.clone();
        let static_direct_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            direct_axpy(pa_backend, 0.25, black_box(&v1), black_box(&mut y_buf));
        });
        let mut y_buf = v2.clone();
        let adhoc_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            AdHocRuntimeEngine::axpy(0.25, black_box(&v1), black_box(&mut y_buf));
        });
        let mut y_buf = v2.clone();
        let platform_aware_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            black_box(&mut y_buf).axpy(0.25, black_box(&v1));
        });
        results.push(OpResult {
            op: "axpy",
            unit: "GB/s",
            work: (3 * p.vec_len * std::mem::size_of::<f32>()) as f64,
            static_cfg_ms,
            static_direct_ms,
            adhoc_ms,
            platform_aware_ms,
            max_abs_diff: diff,
        });
    }

    // --- relu ---
    {
        let diff = v1.relu().max_abs_diff(&ref_relu);
        let static_cfg_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            black_box(StaticCfgEngine::relu(black_box(&v1)));
        });
        let static_direct_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            black_box(direct_relu(pa_backend, black_box(&v1)));
        });
        let adhoc_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            black_box(AdHocRuntimeEngine::relu(black_box(&v1)));
        });
        let platform_aware_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            black_box(black_box(&v1).relu());
        });
        results.push(OpResult {
            op: "relu",
            unit: "GB/s",
            work: (2 * p.vec_len * std::mem::size_of::<f32>()) as f64,
            static_cfg_ms,
            static_direct_ms,
            adhoc_ms,
            platform_aware_ms,
            max_abs_diff: diff,
        });
    }

    // --- matvec ---
    {
        let diff = a_mv.matvec(&x_mv).max_abs_diff(&ref_mv);
        let static_cfg_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            black_box(StaticCfgEngine::matvec(black_box(&a_mv), black_box(&x_mv)));
        });
        let static_direct_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            black_box(direct_matvec(
                pa_backend,
                black_box(&a_mv),
                black_box(&x_mv),
            ));
        });
        let adhoc_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            black_box(AdHocRuntimeEngine::matvec(
                black_box(&a_mv),
                black_box(&x_mv),
            ));
        });
        let platform_aware_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            black_box(black_box(&a_mv).matvec(black_box(&x_mv)));
        });
        results.push(OpResult {
            op: "matvec",
            unit: "GFLOP/s",
            work: (2 * p.mv_m * p.mv_k) as f64,
            static_cfg_ms,
            static_direct_ms,
            adhoc_ms,
            platform_aware_ms,
            max_abs_diff: diff,
        });
    }

    // --- matmul ---
    {
        let diff = a_mm.matmul(&b_mm).max_abs_diff(&ref_mm);
        let static_cfg_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            black_box(StaticCfgEngine::matmul(black_box(&a_mm), black_box(&b_mm)));
        });
        let static_direct_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            black_box(direct_matmul(
                pa_backend,
                black_box(&a_mm),
                black_box(&b_mm),
            ));
        });
        let adhoc_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            black_box(AdHocRuntimeEngine::matmul(
                black_box(&a_mm),
                black_box(&b_mm),
            ));
        });
        let platform_aware_ms = measure_median_ms(cfg.warmup, cfg.iters, || {
            black_box(black_box(&a_mm).matmul(black_box(&b_mm)));
        });
        results.push(OpResult {
            op: "matmul",
            unit: "GFLOP/s",
            work: (2 * p.mm_m * p.mm_k * p.mm_n) as f64,
            static_cfg_ms,
            static_direct_ms,
            adhoc_ms,
            platform_aware_ms,
            max_abs_diff: diff,
        });
    }

    if cfg.csv {
        println!(
            "preset,pa_backend,static_cfg_backend,adhoc_backend,op,static_cfg_ms,static_direct_ms,adhoc_ms,platform_aware_ms,speedup_vs_static_cfg,overhead_vs_direct_pct,pa_throughput,unit,max_abs_diff"
        );
        for r in &results {
            let speedup_cfg = r.static_cfg_ms / r.platform_aware_ms;
            let overhead_pct =
                ((r.platform_aware_ms - r.static_direct_ms) / r.static_direct_ms) * 100.0;
            println!(
                "{},{},{},{},{},{:.4},{:.4},{:.4},{:.4},{:.2},{:+.2},{:.2},{},{:.2e}",
                p.name,
                pa_backend,
                static_cfg_backend,
                adhoc_backend,
                r.op,
                r.static_cfg_ms,
                r.static_direct_ms,
                r.adhoc_ms,
                r.platform_aware_ms,
                speedup_cfg,
                overhead_pct,
                throughput(r.work, r.platform_aware_ms),
                r.unit,
                r.max_abs_diff
            );
        }
        return;
    }

    let profile_src = std::env::var("PLATFORM_DESCRIPTION").unwrap_or_else(|_| "./Platform.toml".into());
    println!("==========================================================================================");
    println!(" Micro Tensor Engine Benchmark — platform-aware vs. Static Approaches");
    println!("==========================================================================================");
    println!(
        " Preset          : {} (vec={}, matvec={}x{}, matmul={}x{}x{})",
        p.name, p.vec_len, p.mv_m, p.mv_k, p.mm_m, p.mm_k, p.mm_n
    );
    println!(" Iterations      : {} warmup / {} timed (median), threads={}", cfg.warmup, cfg.iters, worker_count());
    println!(" Platform.toml   : {}", profile_src);
    println!(
        " Active backends : platform-aware=`{}` | static-cfg=`{}` | ad-hoc=`{}`",
        pa_backend, static_cfg_backend, adhoc_backend
    );
    println!("------------------------------------------------------------------------------------------");
    println!(
        " Dispatch overhead ({} calls): 1st-call resolve = {:.1} us ({:.3} ms)",
        cfg.overhead_calls,
        first_call_us,
        first_call_us / 1_000.0
    );
    println!(
        " Amortized per call             : static-direct = {:.2} ns | platform-aware = {:.2} ns | ad-hoc = {:.2} ns",
        static_probe_ns, pa_probe_ns, adhoc_probe_ns
    );
    println!("------------------------------------------------------------------------------------------");
    println!(
        " {:<8} | {:>10} | {:>10} | {:>10} | {:>10} | {:>9} | {:>8} | {:>13} | {:>8}",
        "Kernel",
        "static-cfg",
        "direct",
        "ad-hoc",
        "plat-aware",
        "vs cfg",
        "ovh%",
        "PA rate",
        "max_err"
    );
    println!(
        " {:<8} | {:>10} | {:>10} | {:>10} | {:>10} | {:>9} | {:>8} | {:>13} | {:>8}",
        "", "(ms)", "(ms)", "(ms)", "(ms)", "(speedup)", "(vs dir)", "", ""
    );
    println!("------------------------------------------------------------------------------------------");
    for r in &results {
        let speedup_cfg = r.static_cfg_ms / r.platform_aware_ms;
        let overhead_pct =
            ((r.platform_aware_ms - r.static_direct_ms) / r.static_direct_ms) * 100.0;
        let rate = throughput(r.work, r.platform_aware_ms);
        println!(
            " {:<8} | {:>10.3} | {:>10.3} | {:>10.3} | {:>10.3} | {:>8.2}x | {:>+7.2}% | {:>5.1} {:<7} | {:>8.1e}",
            r.op,
            r.static_cfg_ms,
            r.static_direct_ms,
            r.adhoc_ms,
            r.platform_aware_ms,
            speedup_cfg,
            overhead_pct,
            rate,
            r.unit,
            r.max_abs_diff
        );
    }
    println!("==========================================================================================");
}
