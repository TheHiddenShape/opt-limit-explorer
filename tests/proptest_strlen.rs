//! Property-based tests: for arbitrary random inputs, every implementation must
//! return exactly the reference length computed by the Rust std.

use libc_bench::strlen::implementations;
use proptest::prelude::*;
use std::ffi::CString;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(2000))]

    #[test]
    fn matches_reference(
        // Non-zero bytes (CString forbids interior 0), length 0..=512.
        content in proptest::collection::vec(1u8..=255, 0..=512),
        // Alignment offset applied to the pointer passed in.
        shift in 0usize..16,
    ) {
        let expected = content.len();

        let mut buf = vec![b'#'; shift];
        buf.extend_from_slice(&content);
        let c = CString::new(buf).unwrap();

        for imp in implementations() {
            // SAFETY: null-terminated string, pointer shifted inside it.
            let got = unsafe { (imp.func)(c.as_ptr().add(shift) as *const u8) };
            prop_assert_eq!(got, expected, "{} failed (shift={})", imp.name, shift);
        }
    }
}
