//! Rust reimplementations of `memcpy` plus adapters to the C versions.
//!
//! As in [`crate::strlen`], all implementations share a common [`MemcpyFn`]
//! signature and are registered in a registry ([`implementations`]) that both
//! tests and benchmarks iterate over.
//!
//! # Contract (same as `memcpy(3)`)
//! `dst` and `src` point to regions of at least `n` bytes, **disjoint** (no
//! overlap). After the call, the first `n` bytes of `dst` equal those of `src`.

use crate::ffi;
use crate::strlen::Lang;
use core::ffi::c_void;

/// Common signature shared by all tested implementations.
///
/// # Safety
/// See the module contract: valid regions of at least `n` bytes, disjoint.
pub type MemcpyFn = unsafe fn(dst: *mut u8, src: *const u8, n: usize);

/// A named implementation, ready to be benchmarked or tested.
#[derive(Clone, Copy)]
pub struct Implementation {
    pub name: &'static str,
    pub lang: Lang,
    pub func: MemcpyFn,
}

/// Returns all known implementations (C + Rust).
pub fn implementations() -> &'static [Implementation] {
    &[
        Implementation { name: "c_glibc", lang: Lang::C, func: glibc },
        Implementation { name: "c_naive", lang: Lang::C, func: c_naive },
        Implementation { name: "rust_naive", lang: Lang::Rust, func: rust_naive },
        Implementation { name: "rust_word", lang: Lang::Rust, func: rust_word },
    ]
}

// ---------------------------------------------------------------------------
// C adapters
// ---------------------------------------------------------------------------

/// Adapter to glibc `memcpy(3)`.
///
/// # Safety
/// See [`MemcpyFn`].
unsafe fn glibc(dst: *mut u8, src: *const u8, n: usize) {
    ffi::memcpy(dst as *mut c_void, src as *const c_void, n);
}

/// Adapter to our naive C memcpy.
///
/// # Safety
/// See [`MemcpyFn`].
unsafe fn c_naive(dst: *mut u8, src: *const u8, n: usize) {
    ffi::c_memcpy_naive(dst as *mut c_void, src as *const c_void, n);
}

// ---------------------------------------------------------------------------
// Rust implementations
// ---------------------------------------------------------------------------

/// Naive scalar memcpy: one byte at a time.
///
/// # Safety
/// See [`MemcpyFn`].
pub unsafe fn rust_naive(dst: *mut u8, src: *const u8, n: usize) {
    let mut i = 0usize;
    while i < n {
        *dst.add(i) = *src.add(i);
        i += 1;
    }
}

const WORD: usize = core::mem::size_of::<usize>();

/// Word-at-a-time memcpy: copies `WORD` bytes per iteration through unaligned
/// reads/writes (`read_unaligned`/`write_unaligned`), then finishes the
/// remaining bytes one at a time.
///
/// # Safety
/// See [`MemcpyFn`].
pub unsafe fn rust_word(dst: *mut u8, src: *const u8, n: usize) {
    let mut i = 0usize;

    // Body: full machine words.
    while i + WORD <= n {
        let w = (src.add(i) as *const usize).read_unaligned();
        (dst.add(i) as *mut usize).write_unaligned(w);
        i += WORD;
    }

    // Tail: leftover bytes (< WORD).
    while i < n {
        *dst.add(i) = *src.add(i);
        i += 1;
    }
}
