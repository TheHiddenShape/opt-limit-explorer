/*
 * Reference C implementations for the `strlen` benchmark.
 *
 * We deliberately keep a naive (scalar, byte-by-byte) version: it is the lower
 * bound we compare glibc (linked separately) and the Rust reimplementations
 * against.
 *
 * Compiled with -fno-builtin to stop GCC/Clang from recognizing the pattern
 * and replacing it with a call to the optimized libc strlen.
 */
#include <stddef.h>

/* Naive scalar strlen: one byte at a time until the null terminator. */
size_t c_strlen_naive(const char *s) {
    const char *p = s;
    while (*p) {
        p++;
    }
    return (size_t)(p - s);
}
