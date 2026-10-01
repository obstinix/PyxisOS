#include <arch/x86_64/interrupts/irq.h>
#include <arch/x86_64/interrupts/pic.h>
#include <pyxis/kernel.h>

static irq_handler_t interrupt_handlers[256];

static const char *exception_messages[32] = {
    "Divide-by-zero Error",
    "Debug",
    "Non-maskable Interrupt",
    "Breakpoint",
    "Overflow",
    "Bound Range Exceeded",
    "Invalid Opcode",
    "Device Not Available",
    "Double Fault",
    "Coprocessor Segment Overrun",
    "Invalid TSS",
    "Segment Not Present",
    "Stack-Segment Fault",
    "General Protection Fault",
    "Page Fault",
    "Reserved",
    "x87 Floating-Point Exception",
    "Alignment Check",
    "Machine Check",
    "SIMD Floating-Point Exception",
    "Virtualization Exception",
    "Control Protection Exception",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Hypervisor Injection Exception",
    "VMM Communication Exception",
    "Security Exception",
    "Reserved"
};

void irq_init(void) {
    memset(interrupt_handlers, 0, sizeof(interrupt_handlers));
}

void register_interrupt_handler(uint8_t vector, irq_handler_t handler) {
    interrupt_handlers[vector] = handler;
}

void irq_dispatch(struct interrupt_frame *frame) {
    if (frame->int_no < 32) {
        kprintf("\n==================== CPU EXCEPTION ====================\n");
        kprintf("Exception [%d]: %s\n", frame->int_no, exception_messages[frame->int_no]);
        kprintf("Error Code : 0x%x\n", frame->err_code);
        kprintf("RIP        : 0x%p  CS : 0x%x\n", frame->rip, frame->cs);
        kprintf("RFLAGS     : 0x%p  RSP: 0x%p  SS: 0x%x\n", frame->rflags, frame->rsp, frame->ss);
        kprintf("RAX: 0x%p  RBX: 0x%p  RCX: 0x%p  RDX: 0x%p\n", frame->rax, frame->rbx, frame->rcx, frame->rdx);
        kprintf("RSI: 0x%p  RDI: 0x%p  RBP: 0x%p\n", frame->rsi, frame->rdi, frame->rbp);
        kprintf("=======================================================\n");

        if (frame->int_no == 14) {
            uint64_t cr2;
            __asm__ volatile ("mov %%cr2, %0" : "=r"(cr2));
            kprintf("Faulting Address (CR2): 0x%p\n", cr2);
        }

        panic(__FILE__, __LINE__, "Unhandled CPU Exception");
    }

    if (interrupt_handlers[frame->int_no]) {
        interrupt_handlers[frame->int_no](frame);
    }

    /* Send EOI for hardware IRQs (vectors 32 - 47) */
    if (frame->int_no >= 32 && frame->int_no < 48) {
        pic_send_eoi((uint8_t)(frame->int_no - 32));
    }
}
