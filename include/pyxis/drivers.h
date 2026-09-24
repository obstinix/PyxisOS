#ifndef PYXIS_DRIVERS_H
#define PYXIS_DRIVERS_H

#include <pyxis/types.h>

void serial_init(void);
void serial_putc(char c);
void serial_puts(const char *str);
char serial_getc(void);
bool serial_has_data(void);

void vga_init(void);
void vga_putc(char c);
void vga_puts(const char *str);
void vga_clear(void);
void vga_set_color(uint8_t fg, uint8_t bg);

void pit_init(uint32_t frequency_hz);
uint64_t pit_get_ticks(void);
uint64_t pit_get_uptime_seconds(void);
void pit_sleep_ms(uint32_t ms);

void keyboard_init(void);
char keyboard_getchar_nonblocking(void);
char keyboard_getchar(void);

#endif
