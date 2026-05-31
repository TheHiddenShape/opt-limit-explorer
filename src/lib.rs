//! `libc-bench` — performance and correctness comparison between libc functions
//! (optimized C glibc, naive C) and Rust reimplementations.
//!
//! The idea: take heavily optimized libc primitives (`strlen`, `memcpy`, …),
//! try to match/beat them in Rust, and **prove** through rigorous tests that
//! the behavior is strictly identical before comparing timings.
//!
//! - [`strlen`], [`memcpy`]: implementation registries and Rust code.
//! - Benchmarks live in `benches/`, tests in `tests/`.

pub mod ffi;
pub mod memcpy;
pub mod strlen;
