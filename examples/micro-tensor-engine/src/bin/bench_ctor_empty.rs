//! Minimal empty binary without `platform-aware` (0 `#[ctor]` taxonomy constructors).
//! Paired with `bench_ctor_pa` to directly isolate pre-`main()` `#[ctor]` startup cost.

fn main() {
    std::hint::black_box(0usize);
}
