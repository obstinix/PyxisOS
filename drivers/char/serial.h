#ifndef PYXIS_DRIVERS_SERIAL_H
#define PYXIS_DRIVERS_SERIAL_H

#include <pyxis/types.h>

#define COM1_PORT 0x3F8

void serial_init(void);
void serial_putc(char c);
void serial_puts(const char *str);
char serial_getc(void);
bool serial_has_data(void);

#endif
