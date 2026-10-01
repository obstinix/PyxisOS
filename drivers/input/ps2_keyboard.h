#ifndef PYXIS_DRIVERS_PS2_KEYBOARD_H
#define PYXIS_DRIVERS_PS2_KEYBOARD_H

#include <pyxis/types.h>

void keyboard_init(void);
char keyboard_getchar_nonblocking(void);
char keyboard_getchar(void);

#endif
