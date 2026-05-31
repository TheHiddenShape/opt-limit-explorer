//! FFI declarations to the C code.
//!
//! - `strlen`: the real system libc (glibc) implementation, usually vectorized
//!   (SSE/AVX) and written in assembly. This is the target to beat.
//! - `c_strlen_naive`: our naive C implementation, compiled by `build.rs`.

use core::ffi::c_char;

extern "C" {
    /// System libc `strlen(3)`.
    pub fn strlen(s: *const c_char) -> usize;

    /// Our naive C strlen (see `c_src/strlen.c`).
    pub fn c_strlen_naive(s: *const c_char) -> usize;
}
