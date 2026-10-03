// Reads memory that may not be mapped, without crashing.
//
// Rust has no way to handle structured exceptions, so the read happens here,
// under `__try`/`__except`. The exception is caught in this frame and never
// unwinds into Rust code. On x64, SEH is table-based: a successful read costs
// no more than the copy itself.
//
// Two constraints decide how the copy is done:
// - Without asynchronous exception support (`/EHa`), clang only catches
//   exceptions raised in functions *called* from the `__try` block, so the
//   copy must be a call.
// - The unwinder must be able to walk out of the callee to find the handler,
//   which isn't always the case for the CRT's `memcpy` (e.g. Wine's builtin
//   one), so the callee is our own. Its loads are volatile so the compiler
//   can't turn the loop into a `memcpy` call.

#include <excpt.h>
#include <stddef.h>

#define STATUS_ACCESS_VIOLATION 0xC0000005UL

__declspec(noinline) static void copy_volatile(const void *src, void *dst, size_t len) {
    const unsigned char *s = (const unsigned char *)src;
    unsigned char *d = (unsigned char *)dst;

    for (; len >= 8; len -= 8, s += 8, d += 8) {
        *(unsigned __int64 __unaligned *)d = *(const volatile unsigned __int64 __unaligned *)s;
    }
    for (; len > 0; len--, s++, d++) {
        *d = *(const volatile unsigned char *)s;
    }
}

// Copies `len` bytes from `src` to `dst`. Returns 1 on success, 0 if reading
// `src` raised an access violation; `dst` may then be partially written.
// Other exceptions (e.g. guard page hits) are not handled.
int libds3_guarded_read(const void *src, void *dst, size_t len) {
    __try {
        copy_volatile(src, dst, len);
        return 1;
    } __except (GetExceptionCode() == STATUS_ACCESS_VIOLATION ? EXCEPTION_EXECUTE_HANDLER
                                                              : EXCEPTION_CONTINUE_SEARCH) {
        return 0;
    }
}
