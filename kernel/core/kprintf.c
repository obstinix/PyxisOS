#include <pyxis/kernel.h>
#include <drivers/char/serial.h>
#include <drivers/video/vga.h>
#include <stdarg.h>

void kputc(char c) {
    serial_putc(c);
    vga_putc(c);
}

void kputs(const char *str) {
    while (str && *str) {
        kputc(*str++);
    }
}

static void print_dec(uint64_t val) {
    if (val == 0) {
        kputc('0');
        return;
    }
    char buf[32];
    int i = 0;
    while (val > 0) {
        buf[i++] = (char)('0' + (val % 10));
        val /= 10;
    }
    while (--i >= 0) {
        kputc(buf[i]);
    }
}

static void print_hex(uint64_t val) {
    if (val == 0) {
        kputc('0');
        return;
    }
    char buf[32];
    const char *hex_digits = "0123456789abcdef";
    int i = 0;
    while (val > 0) {
        buf[i++] = hex_digits[val & 0x0F];
        val >>= 4;
    }
    while (--i >= 0) {
        kputc(buf[i]);
    }
}

void kprintf(const char *fmt, ...) {
    va_list args;
    va_start(args, fmt);

    while (fmt && *fmt) {
        if (*fmt == '%') {
            fmt++;
            switch (*fmt) {
                case 'c': {
                    char c = (char)va_arg(args, int);
                    kputc(c);
                    break;
                }
                case 's': {
                    const char *s = va_arg(args, const char *);
                    kputs(s ? s : "(null)");
                    break;
                }
                case 'd': {
                    int64_t d = va_arg(args, int64_t);
                    if (d < 0) {
                        kputc('-');
                        d = -d;
                    }
                    print_dec((uint64_t)d);
                    break;
                }
                case 'u': {
                    uint64_t u = va_arg(args, uint64_t);
                    print_dec(u);
                    break;
                }
                case 'x': {
                    uint64_t x = va_arg(args, uint64_t);
                    print_hex(x);
                    break;
                }
                case 'p': {
                    uint64_t p = (uint64_t)va_arg(args, void *);
                    kputs("0x");
                    print_hex(p);
                    break;
                }
                case '%': {
                    kputc('%');
                    break;
                }
                default: {
                    kputc('%');
                    kputc(*fmt);
                    break;
                }
            }
        } else {
            kputc(*fmt);
        }
        fmt++;
    }

    va_end(args);
}
