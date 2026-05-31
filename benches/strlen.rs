//! Criterion benchmark: compares all `strlen` implementations across a range of
//! input sizes, measuring throughput (bytes/s).
//!
//! Run: `cargo bench --bench strlen`
//! HTML reports: `target/criterion/report/index.html`

use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use libc_bench::strlen::implementations;
use std::ffi::CString;

/// Tested sizes: short (fixed cost dominates) → long (pure throughput).
const SIZES: &[usize] = &[7, 16, 64, 256, 1024, 4096, 65536];

fn bench_strlen(c: &mut Criterion) {
    let mut group = c.benchmark_group("strlen");

    for &size in SIZES {
        // String of `size` non-zero bytes, null terminated (CString).
        let data = CString::new(vec![b'a'; size]).unwrap();
        let ptr = data.as_ptr() as *const u8;

        group.throughput(Throughput::Bytes(size as u64));

        for imp in implementations() {
            group.bench_with_input(
                BenchmarkId::new(imp.name, size),
                &ptr,
                |b, &ptr| {
                    // SAFETY: `data` stays alive for the whole benchmark.
                    b.iter(|| unsafe { (imp.func)(black_box(ptr)) });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(benches, bench_strlen);
criterion_main!(benches);
