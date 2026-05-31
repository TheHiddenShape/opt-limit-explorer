# libc-bench

Precise and **verified** benchmarking of `libc` primitives: we compare the
optimized `glibc` version, a naive C version, and Rust reimplementations.

The golden rule: we **never** compare two functions without first proving they
behave identically.

## Architecture

```
c_src/*.c             Reference C implementations (naive), compiled by build.rs
build.rs              Compiles the C (-O3 -march=native -fno-builtin) and links it
src/ffi.rs            FFI declarations: glibc strlen/memcpy + our C functions
src/strlen.rs         strlen Rust reimplementations + common impl registry
src/memcpy.rs         memcpy Rust reimplementations + common impl registry
benches/*.rs          Criterion benchmarks (throughput per input size)
tests/*.rs            Correctness tests (edge cases, alignments) + property-based
```

Each implementation (C or Rust) is exposed through a common signature
(`StrlenFn` / `MemcpyFn`) and registered in `<fn>::implementations()`. Tests
**and** benchmarks iterate over that same registry: adding an implementation
means adding it in a single place.

Current implementations:

| function | name         | lang | description                              |
|----------|--------------|------|------------------------------------------|
| strlen   | `c_glibc`    | C    | libc `strlen(3)` (perf reference)        |
| strlen   | `c_naive`    | C    | scalar byte-by-byte loop                 |
| strlen   | `rust_naive` | Rust | scalar byte-by-byte loop                 |
| strlen   | `rust_swar`  | Rust | word-at-a-time (SWAR bitwise trick)      |
| memcpy   | `c_glibc`    | C    | libc `memcpy(3)` (perf reference)        |
| memcpy   | `c_naive`    | C    | scalar byte-by-byte loop                 |
| memcpy   | `rust_naive` | Rust | scalar byte-by-byte loop                 |
| memcpy   | `rust_word`  | Rust | word-at-a-time (unaligned word copies)   |

## Usage

```sh
cargo test                  # correctness (edge cases + property-based)
cargo bench --bench strlen  # benchmarks -> target/criterion/report/index.html
cargo bench --bench memcpy
```

> Note: `cargo bench` without `--bench <name>` also runs the lib unittest
> binary, which does not understand Criterion CLI flags. Target a specific
> bench when passing flags.

## Adding a function (strcmp, memchr, …)

1. Reference C code in `c_src/`, declared in `src/ffi.rs` and compiled via `build.rs`.
2. Rust reimplementation(s) in a new `src/<fn>.rs` module with an
   `implementations()`.
3. Reuse the `tests/` and `benches/` scheme (edge cases + proptest + Criterion).
