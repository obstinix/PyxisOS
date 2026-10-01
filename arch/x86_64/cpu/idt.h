#ifndef PYXIS_ARCH_IDT_H
#define PYXIS_ARCH_IDT_H

#include <pyxis/types.h>

struct idt_entry {
    uint16_t offset_low;
    uint16_t selector;
    uint8_t  ist;
    uint8_t  type_attr;
    uint16_t offset_mid;
    uint32_t offset_high;
    uint32_t zero;
} PACKED;

struct idt_ptr {
    uint16_t limit;
    uint64_t base;
} PACKED;

struct interrupt_frame {
    uint64_t r15, r14, r13, r12, r11, r10, r9, r8;
    uint64_t rbp, rdi, rsi, rdx, rcx, rbx, rax;
    uint64_t int_no, err_code;
    uint64_t rip, cs, rflags, rsp, ss;
} PACKED;

typedef void (*irq_handler_t)(struct interrupt_frame *frame);

void idt_init(void);
void idt_set_gate(uint8_t num, uint64_t base, uint16_t sel, uint8_t flags, uint8_t ist);

#endif
