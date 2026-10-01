#ifndef PYXIS_ARCH_GDT_H
#define PYXIS_ARCH_GDT_H

#include <pyxis/types.h>

#define GDT_KERNEL_CODE_SEG 0x08
#define GDT_KERNEL_DATA_SEG 0x10
#define GDT_USER_DATA_SEG   0x18
#define GDT_USER_CODE_SEG   0x20
#define GDT_TSS_SEG         0x28

struct tss_entry {
    uint32_t reserved0;
    uint64_t rsp0;
    uint64_t rsp1;
    uint64_t rsp2;
    uint64_t reserved1;
    uint64_t ist1;
    uint64_t ist2;
    uint64_t ist3;
    uint64_t ist4;
    uint64_t ist5;
    uint64_t ist6;
    uint64_t ist7;
    uint64_t reserved2;
    uint16_t reserved3;
    uint16_t iomap_base;
} PACKED;

struct gdt_entry {
    uint16_t limit_low;
    uint16_t base_low;
    uint8_t  base_middle;
    uint8_t  access;
    uint8_t  granularity;
    uint8_t  base_high;
} PACKED;

struct gdt_ptr {
    uint16_t limit;
    uint64_t base;
} PACKED;

void gdt_init(void);
void gdt_set_kernel_stack(uint64_t stack);

#endif
