//! FFI declarations to the C code.
//!
//! - `strlen` / `memcpy`: the real system libc (glibc) implementations, usually
//!   vectorized (SSE/AVX) and written in assembly. These are the targets to beat.
//! - `c_*_naive`: our naive C implementations, compiled by `build.rs`.

use core::ffi::{c_char, c_void};

extern "C" {
    /// System libc `strlen(3)`.
    pub fn strlen(s: *const c_char) -> usize;

    /// Our naive C strlen (see `c_src/strlen.c`).
    pub fn c_strlen_naive(s: *const c_char) -> usize;

    /// System libc `memcpy(3)` (non-overlapping regions).
    pub fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;

    /// Our naive C memcpy (see `c_src/memcpy.c`).
    pub fn c_memcpy_naive(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}
