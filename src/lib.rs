//! `libc-bench` — performance and correctness comparison between libc functions
//! (optimized C glibc, naive C) and Rust reimplementations.
//!
//! The idea: take heavily optimized libc primitives (`strlen`, …), try to
//! match/beat them in Rust, and **prove** through rigorous tests that the
//! behavior is strictly identical before comparing timings.
//!
//! - [`strlen`]: implementation registry and Rust code.
//! - Benchmarks live in `benches/`, tests in `tests/`.

pub mod ffi;
pub mod strlen;
