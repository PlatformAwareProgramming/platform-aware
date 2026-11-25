//! Minimal binary linking `platform-aware` and its vendor taxonomy crates (`#[ctor]` active)
//! while executing an empty `main()` (never calling `resolve()` unless `--call` is passed).
//! Directly isolates pre-`main()` `#[ctor]` startup cost vs. `bench_ctor_empty`.

use micro_tensor_engine::platform_engine::ops::dispatch_probe;

fn main() {
    if std::env::args().any(|a| a == "--call") {
        std::hint::black_box(dispatch_probe(0));
    } else {
        // Keep the symbol reachable so LTO cannot strip platform-aware `#[ctor]` initializers.
        let f: fn(u64) -> u64 = dispatch_probe;
        std::hint::black_box(f as usize);
    }
}
