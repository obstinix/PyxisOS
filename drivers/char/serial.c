#include <drivers/char/serial.h>
#include <pyxis/kernel.h>

void serial_init(void) {
    outb(COM1_PORT + 1, 0x00); /* Disable all interrupts */
    outb(COM1_PORT + 3, 0x80); /* Enable DLAB (set baud rate divisor) */
    outb(COM1_PORT + 0, 0x03); /* Set divisor to 3 (38400 baud) - low byte */
    outb(COM1_PORT + 1, 0x00); /* High byte */
    outb(COM1_PORT + 3, 0x03); /* 8 bits, no parity, one stop bit */
    outb(COM1_PORT + 2, 0xC7); /* Enable FIFO, clear them, with 14-byte threshold */
    outb(COM1_PORT + 4, 0x0B); /* IRQs enabled, RTS/DSR set */
}

static int is_transmit_empty(void) {
    return inb(COM1_PORT + 5) & 0x20;
}

void serial_putc(char c) {
    while (is_transmit_empty() == 0) {}
    outb(COM1_PORT, (uint8_t)c);
}

void serial_puts(const char *str) {
    while (str && *str) {
        if (*str == '\n') {
            serial_putc('\r');
        }
        serial_putc(*str++);
    }
}

bool serial_has_data(void) {
    return (inb(COM1_PORT + 5) & 1) != 0;
}

char serial_getc(void) {
    while (!serial_has_data()) {}
    return (char)inb(COM1_PORT);
}
