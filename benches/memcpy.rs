//! Criterion benchmark: compares all `memcpy` implementations across a range of
//! sizes, measuring throughput (bytes/s).
//!
//! Run: `cargo bench --bench memcpy`
//! HTML reports: `target/criterion/report/index.html`

use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use libc_bench::memcpy::implementations;

/// Tested sizes: short (fixed cost dominates) → long (pure throughput).
const SIZES: &[usize] = &[8, 16, 64, 256, 1024, 4096, 65536];

fn bench_memcpy(c: &mut Criterion) {
    let mut group = c.benchmark_group("memcpy");

    for &size in SIZES {
        let src = vec![0xABu8; size];
        // Destination buffer reused across iterations.
        let mut dst = vec![0u8; size];

        group.throughput(Throughput::Bytes(size as u64));

        for imp in implementations() {
            group.bench_with_input(
                BenchmarkId::new(imp.name, size),
                &size,
                |b, &size| {
                    b.iter(|| unsafe {
                        // SAFETY: src and dst are `size` bytes, disjoint regions.
                        (imp.func)(
                            black_box(dst.as_mut_ptr()),
                            black_box(src.as_ptr()),
                            black_box(size),
                        );
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(benches, bench_memcpy);
criterion_main!(benches);
