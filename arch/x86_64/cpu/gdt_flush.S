.code64
.global gdt_flush
.global tss_flush

gdt_flush:
    lgdt (%rdi)

    movw $0x10, %ax
    movw %ax, %ds
    movw %ax, %es
    movw %ax, %fs
    movw %ax, %gs
    movw %ax, %ss

    /* Far return to reload CS */
    pushq $0x08
    leaq .reload_cs(%rip), %rax
    pushq %rax
    lretq

.reload_cs:
    ret

tss_flush:
    movw %di, %ax
    ltr %ax
    ret
