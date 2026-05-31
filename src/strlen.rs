//! Rust reimplementations of `strlen` plus adapters to the C versions.
//!
//! All functions share the same low-level [`StrlenFn`] signature so they can be
//! registered in a common registry (see [`implementations`]) and fed to the
//! same bench/test harness.
//!
//! # Contract (same as `strlen(3)`)
//! `s` must point to a sequence of bytes terminated by a `0`, readable up to and
//! including the terminator. The returned value is the number of bytes before
//! the `0`.

use crate::ffi;
use core::ffi::c_char;

/// Common signature shared by all tested implementations.
///
/// # Safety
/// See the module contract: `s` must be non-null and point to a string
/// terminated by `0`, readable up to the terminator.
pub type StrlenFn = unsafe fn(s: *const u8) -> usize;

/// Source language of an implementation (handy for bench display).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    C,
    Rust,
}

/// A named implementation, ready to be benchmarked or tested.
#[derive(Clone, Copy)]
pub struct Implementation {
    pub name: &'static str,
    pub lang: Lang,
    pub func: StrlenFn,
}

/// Returns all known implementations (C + Rust).
///
/// The first one (`c_glibc`) serves as the performance reference; all of them
/// must be functionally equivalent (verified by the tests).
pub fn implementations() -> &'static [Implementation] {
    &[
        Implementation { name: "c_glibc", lang: Lang::C, func: glibc },
        Implementation { name: "c_naive", lang: Lang::C, func: c_naive },
        Implementation { name: "rust_naive", lang: Lang::Rust, func: rust_naive },
        Implementation { name: "rust_swar", lang: Lang::Rust, func: rust_swar },
    ]
}

// ---------------------------------------------------------------------------
// C adapters
// ---------------------------------------------------------------------------

/// Adapter to glibc `strlen(3)`.
///
/// # Safety
/// See [`StrlenFn`].
unsafe fn glibc(s: *const u8) -> usize {
    ffi::strlen(s as *const c_char)
}

/// Adapter to our naive C strlen.
///
/// # Safety
/// See [`StrlenFn`].
unsafe fn c_naive(s: *const u8) -> usize {
    ffi::c_strlen_naive(s as *const c_char)
}

// ---------------------------------------------------------------------------
// Rust implementations
// ---------------------------------------------------------------------------

/// Naive scalar strlen: one byte at a time.
///
/// # Safety
/// See [`StrlenFn`].
pub unsafe fn rust_naive(s: *const u8) -> usize {
    let mut n = 0usize;
    while *s.add(n) != 0 {
        n += 1;
    }
    n
}

const WORD: usize = core::mem::size_of::<usize>();
/// `0x0101…01`: a 1 in the low bit of every byte.
const LO: usize = usize::MAX / 0xff;
/// `0x8080…80`: a 1 in the high bit of every byte.
const HI: usize = LO << 7;

/// True if `x` contains at least one zero byte (classic SWAR trick).
#[inline(always)]
fn has_zero_byte(x: usize) -> bool {
    x.wrapping_sub(LO) & !x & HI != 0
}

/// Word-at-a-time (SWAR) strlen: processes `WORD` bytes per iteration with a
/// bitwise trick, after aligning the pointer to a word boundary.
///
/// Reading a full word cannot cross a page boundary beyond the word that
/// contains the `0`, because the reads are aligned: we never read past the word
/// that already holds the terminator.
///
/// # Safety
/// See [`StrlenFn`].
pub unsafe fn rust_swar(s: *const u8) -> usize {
    let mut p = s;

    // Unaligned prefix: byte by byte until a word boundary.
    while (p as usize) % WORD != 0 {
        if *p == 0 {
            return p.offset_from(s) as usize;
        }
        p = p.add(1);
    }

    // Aligned body: one machine word at a time.
    loop {
        let word = (p as *const usize).read();
        if has_zero_byte(word) {
            // The word contains a 0: locate the exact byte.
            for i in 0..WORD {
                if *p.add(i) == 0 {
                    return p.add(i).offset_from(s) as usize;
                }
            }
        }
        p = p.add(WORD);
    }
}
