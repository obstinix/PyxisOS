#include <pyxis/types.h>
#include <pyxis/kernel.h>
#include <drivers/timer/pit.h>
#include <include/uapi/syscalls.h>

int64_t syscall_handler(uint64_t num, uint64_t arg1, uint64_t arg2, uint64_t arg3) {
    (void)arg1; (void)arg2; (void)arg3;
    switch (num) {
        case SYS_EXIT:
            return 0;
        case SYS_WRITE:
            if (arg2 != 0 && arg3 > 0) {
                const char *buf = (const char *)arg2;
                for (size_t i = 0; i < arg3; i++) {
                    kputc(buf[i]);
                }
                return (int64_t)arg3;
            }
            return -1;
        case SYS_UPTIME:
            return (int64_t)pit_get_uptime_seconds();
        case SYS_YIELD:
            hlt();
            return 0;
        default:
            kprintf("[SYSCALL] Unhandled syscall number: %d\n", (uint32_t)num);
            return -1;
    }
}
