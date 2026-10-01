#include <arch/x86_64/cpu/gdt.h>
#include <pyxis/kernel.h>

extern void gdt_flush(struct gdt_ptr *ptr);
extern void tss_flush(uint16_t sel);

static struct {
    struct gdt_entry null_desc;
    struct gdt_entry kernel_code;
    struct gdt_entry kernel_data;
    struct gdt_entry user_data;
    struct gdt_entry user_code;
    struct gdt_entry tss_low;
    struct gdt_entry tss_high;
} PACKED gdt_table;

static struct gdt_ptr gdt_descriptor;
static struct tss_entry kernel_tss;

static void set_gdt_gate(struct gdt_entry *gate, uint32_t base, uint32_t limit, uint8_t access, uint8_t gran) {
    gate->base_low = (base & 0xFFFF);
    gate->base_middle = (base >> 16) & 0xFF;
    gate->base_high = (base >> 24) & 0xFF;
    gate->limit_low = (limit & 0xFFFF);
    gate->granularity = ((limit >> 16) & 0x0F) | (gran & 0xF0);
    gate->access = access;
}

void gdt_init(void) {
    memset(&gdt_table, 0, sizeof(gdt_table));
    memset(&kernel_tss, 0, sizeof(kernel_tss));

    /* Null Descriptor: 0x00 */
    set_gdt_gate(&gdt_table.null_desc, 0, 0, 0, 0);

    /* Kernel Code 64-bit: 0x08 (Access: 0x9A, Granularity: 0xA0) */
    set_gdt_gate(&gdt_table.kernel_code, 0, 0xFFFFF, 0x9A, 0xA0);

    /* Kernel Data 64-bit: 0x10 (Access: 0x92, Granularity: 0xC0) */
    set_gdt_gate(&gdt_table.kernel_data, 0, 0xFFFFF, 0x92, 0xC0);

    /* User Data 64-bit: 0x18 (Access: 0xF2, Granularity: 0xC0) */
    set_gdt_gate(&gdt_table.user_data, 0, 0xFFFFF, 0xF2, 0xC0);

    /* User Code 64-bit: 0x20 (Access: 0xFA, Granularity: 0xA0) */
    set_gdt_gate(&gdt_table.user_code, 0, 0xFFFFF, 0xFA, 0xA0);

    /* TSS Descriptor: 0x28 (16-byte system descriptor in long mode) */
    uint64_t tss_base = (uint64_t)&kernel_tss;
    uint32_t tss_limit = sizeof(kernel_tss) - 1;

    set_gdt_gate(&gdt_table.tss_low, (uint32_t)tss_base, tss_limit, 0x89, 0x00);

    /* High 32 bits of TSS base into second slot */
    uint32_t *tss_high_ptr = (uint32_t *)&gdt_table.tss_high;
    tss_high_ptr[0] = (uint32_t)(tss_base >> 32);
    tss_high_ptr[1] = 0;

    kernel_tss.iomap_base = sizeof(kernel_tss);

    gdt_descriptor.limit = sizeof(gdt_table) - 1;
    gdt_descriptor.base = (uint64_t)&gdt_table;

    gdt_flush(&gdt_descriptor);
    tss_flush(GDT_TSS_SEG);
}

void gdt_set_kernel_stack(uint64_t stack) {
    kernel_tss.rsp0 = stack;
}
