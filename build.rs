//! Compiles the reference C implementations and links them into the crate.
//!
//! Each C function is compiled at a high optimization level so it is a credible
//! opponent to the Rust versions. The glibc (hand-tuned / SIMD strlen, memcpy)
//! is linked separately through the system's `extern` symbols.

fn main() {
    println!("cargo:rerun-if-changed=c_src/strlen.c");
    println!("cargo:rerun-if-changed=c_src/memcpy.c");

    let mut build = cc::Build::new();
    build
        .file("c_src/strlen.c")
        .file("c_src/memcpy.c")
        .opt_level(3)
        .flag_if_supported("-march=native")
        .flag_if_supported("-fno-builtin") // keep the compiler from replacing
        .warnings(true); //   our loops with a libc call.

    build.compile("cbench");
}
