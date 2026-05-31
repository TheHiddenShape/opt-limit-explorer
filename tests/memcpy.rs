//! Correctness tests for `memcpy`: every implementation must produce exactly
//! the same copy as the source content, without overflowing, across all cases
//! (sizes at word boundaries, alignment offsets).

use libc_bench::memcpy::{implementations, MemcpyFn};

/// Copies `src` through `f` into a guarded buffer, and checks that:
/// - the `n` copied bytes are correct,
/// - nothing was written outside the destination region.
fn check(name: &str, f: MemcpyFn, src: &[u8], dst_shift: usize, src_shift: usize) {
    let n = src.len();
    const GUARD: u8 = 0xCD;
    const PAD: usize = 16;

    // Destination buffer with leading/trailing guards and an alignment offset.
    let mut dst = vec![GUARD; PAD + dst_shift + n + PAD];
    let dst_start = PAD + dst_shift;

    // Source buffer shifted to exercise read-side alignments.
    let mut src_buf = vec![0u8; src_shift + n];
    src_buf[src_shift..].copy_from_slice(src);

    // SAFETY: dst has `n` free bytes from dst_start, src_buf has `n` from
    // src_shift, regions are disjoint (two distinct allocations).
    unsafe {
        f(
            dst.as_mut_ptr().add(dst_start),
            src_buf.as_ptr().add(src_shift),
            n,
        );
    }

    // Copied content is correct.
    assert_eq!(&dst[dst_start..dst_start + n], src, "{name}: content (n={n})");

    // Guards intact before and after.
    assert!(
        dst[..dst_start].iter().all(|&b| b == GUARD),
        "{name}: overflow before (n={n})"
    );
    assert!(
        dst[dst_start + n..].iter().all(|&b| b == GUARD),
        "{name}: overflow after (n={n})"
    );
}

#[test]
fn edge_cases() {
    let lengths = [0, 1, 2, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 4096];

    for imp in implementations() {
        for &len in &lengths {
            // Varied, deterministic content.
            let src: Vec<u8> = (0..len).map(|i| (i as u8).wrapping_mul(31).wrapping_add(7)).collect();
            for dshift in 0..8 {
                for sshift in 0..8 {
                    check(imp.name, imp.func, &src, dshift, sshift);
                }
            }
        }
    }
}

#[test]
fn zero_length_is_noop() {
    // n=0 must touch nothing (including pointers at the boundary).
    for imp in implementations() {
        check(imp.name, imp.func, &[], 0, 0);
    }
}
