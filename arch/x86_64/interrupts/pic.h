#ifndef PYXIS_ARCH_PIC_H
#define PYXIS_ARCH_PIC_H

#include <pyxis/types.h>

#define PIC1_CMD  0x20
#define PIC1_DATA 0x21
#define PIC2_CMD  0xA0
#define PIC2_DATA 0xA1

#define PIC_EOI   0x20

void pic_init(void);
void pic_send_eoi(uint8_t irq);
void pic_set_mask(uint8_t irq_line);
void pic_clear_mask(uint8_t irq_line);
void pic_disable(void);

#endif
