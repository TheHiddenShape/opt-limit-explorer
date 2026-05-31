//! Property-based tests for `memcpy`: for random inputs and offsets, every
//! implementation must copy exactly the source content without overflowing the
//! destination region.

use libc_bench::memcpy::implementations;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(2000))]

    #[test]
    fn copies_exactly(
        src in proptest::collection::vec(any::<u8>(), 0..=512),
        dst_shift in 0usize..16,
        src_shift in 0usize..16,
    ) {
        let n = src.len();
        const GUARD: u8 = 0xCD;
        const PAD: usize = 16;

        let mut dst = vec![GUARD; PAD + dst_shift + n + PAD];
        let dst_start = PAD + dst_shift;

        let mut src_buf = vec![0u8; src_shift + n];
        src_buf[src_shift..].copy_from_slice(&src);

        for imp in implementations() {
            // Reset the destination region before each implementation.
            for b in dst.iter_mut() { *b = GUARD; }

            // SAFETY: valid, disjoint regions of `n` bytes.
            unsafe {
                (imp.func)(
                    dst.as_mut_ptr().add(dst_start),
                    src_buf.as_ptr().add(src_shift),
                    n,
                );
            }

            prop_assert_eq!(&dst[dst_start..dst_start + n], &src[..], "{} content", imp.name);
            prop_assert!(dst[..dst_start].iter().all(|&b| b == GUARD), "{} before", imp.name);
            prop_assert!(dst[dst_start + n..].iter().all(|&b| b == GUARD), "{} after", imp.name);
        }
    }
}
