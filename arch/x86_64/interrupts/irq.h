#ifndef PYXIS_ARCH_IRQ_H
#define PYXIS_ARCH_IRQ_H

#include <arch/x86_64/cpu/idt.h>

void irq_init(void);
void register_interrupt_handler(uint8_t vector, irq_handler_t handler);
void irq_dispatch(struct interrupt_frame *frame);

#endif
