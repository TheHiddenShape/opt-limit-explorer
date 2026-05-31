/*
 * Reference C implementations for the `memcpy` benchmark.
 *
 * Naive version (byte by byte) serving as the lower bound against the optimized
 * glibc memcpy (linked separately) and the Rust reimplementations.
 *
 * Compiled with -fno-builtin to stop the compiler from recognizing the pattern
 * and replacing it with a call to the optimized libc memcpy.
 */
#include <stddef.h>

/* Naive scalar memcpy: copies one byte at a time. The regions must not overlap
 * (same contract as memcpy(3)). */
void *c_memcpy_naive(void *dst, const void *src, size_t n) {
    unsigned char *d = (unsigned char *)dst;
    const unsigned char *s = (const unsigned char *)src;
    for (size_t i = 0; i < n; i++) {
        d[i] = s[i];
    }
    return dst;
}
