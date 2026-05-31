//! Correctness tests: every implementation must behave EXACTLY like the
//! reference (`CStr`/glibc) in all cases, otherwise the benchmark is meaningless.

use libc_bench::strlen::{implementations, StrlenFn};
use std::ffi::CString;

/// Builds a valid C string (no interior `0`) and calls `f` on it.
fn run(f: StrlenFn, bytes: &[u8]) -> usize {
    let c = CString::new(bytes).expect("no interior 0 in test input");
    // SAFETY: `c` is null-terminated and alive for the whole call.
    unsafe { f(c.as_ptr() as *const u8) }
}

/// Checks an implementation against the expected length at several alignment
/// offsets (to trip up word-at-a-time versions).
fn check_with_offsets(name: &str, f: StrlenFn, content: &[u8]) {
    let expected = content.len();

    // Direct case.
    assert_eq!(run(f, content), expected, "{name}: aligned case");

    // Shifted cases: prefix some padding then point past it, to exercise every
    // possible alignment remainder (0..=WORD).
    for shift in 0..16usize {
        let mut buf = vec![b'#'; shift];
        buf.extend_from_slice(content);
        let c = CString::new(buf).unwrap();
        // SAFETY: shifted pointer still inside a null-terminated string.
        let got = unsafe { f(c.as_ptr().add(shift) as *const u8) };
        assert_eq!(got, expected, "{name}: shift {shift}");
    }
}

#[test]
fn edge_cases() {
    // Lengths chosen around word (8) and vector (16/32) boundaries.
    let lengths = [0, 1, 2, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 4095, 4096];

    for imp in implementations() {
        for &len in &lengths {
            let content = vec![b'a'; len];
            check_with_offsets(imp.name, imp.func, &content);
        }
    }
}

#[test]
fn non_ascii_and_high_bytes() {
    // Bytes >= 0x80: trip up poorly written bitwise tricks.
    for imp in implementations() {
        let content: Vec<u8> = (1u8..=255).cycle().take(300).collect();
        check_with_offsets(imp.name, imp.func, &content);
    }
}

#[test]
fn all_implementations_agree() {
    // Self-consistency: all implementations return the same thing.
    let imps = implementations();
    let samples: Vec<Vec<u8>> = (0..200)
        .map(|n| (1u8..=255).cycle().take(n).collect())
        .collect();

    for content in &samples {
        let c = CString::new(content.clone()).unwrap();
        let reference = unsafe { (imps[0].func)(c.as_ptr() as *const u8) };
        for imp in imps {
            let got = unsafe { (imp.func)(c.as_ptr() as *const u8) };
            assert_eq!(got, reference, "{} diverges", imp.name);
        }
    }
}
