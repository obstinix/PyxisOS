/* PyxisOS x86_64 Bootstrap */
.code32
.section .boot, "ax"
.global _start
.extern kernel_main

_start:
    cli
    cld

    movl %eax, mb_magic
    movl %ebx, mb_info

    /* Setup early stack */
    movl $stack_top, %esp

    /* Verify CPUID support */
    pushfl
    popl %eax
    movl %eax, %ecx
    xorl $0x200000, %eax
    pushl %eax
    popfl
    pushfl
    popl %eax
    pushl %ecx
    popfl
    cmpl %ecx, %eax
    je .no_cpuid

    /* Verify Long Mode support (EFER.LME) */
    movl $0x80000000, %eax
    cpuid
    cmpl $0x80000001, %eax
    jb .no_long_mode

    movl $0x80000001, %eax
    cpuid
    testl $(1 << 29), %edx
    jz .no_long_mode

    /* Build early paging tables (Identity map first 1GB with 2MB huge pages) */
    /* PML4[0] -> PDPT */
    movl $boot_pdpt, %eax
    orl $0x03, %eax /* Present + Writable */
    movl %eax, boot_pml4

    /* PDPT[0] -> PD */
    movl $boot_pd, %eax
    orl $0x03, %eax /* Present + Writable */
    movl %eax, boot_pdpt

    /* Identity map first 64MB using 32 x 2MB pages */
    movl $0, %ecx
.map_pd:
    movl %ecx, %eax
    shll $21, %eax      /* Multiply by 2MB */
    orl $0x83, %eax     /* Present + Writable + Huge Page (bit 7) */
    movl %eax, boot_pd(,%ecx,8)
    incl %ecx
    cmpl $32, %ecx
    jne .map_pd

    /* Enable PAE (Physical Address Extension) in CR4 */
    movl %cr4, %eax
    orl $(1 << 5), %eax
    movl %eax, %cr4

    /* Load CR3 with physical address of PML4 */
    movl $boot_pml4, %eax
    movl %eax, %cr3

    /* Enable Long Mode in EFER MSR */
    movl $0xC0000080, %ecx
    rdmsr
    orl $(1 << 8), %eax /* LME flag */
    wrmsr

    /* Enable Paging and Protected Mode in CR0 */
    movl %cr0, %eax
    orl $((1 << 31) | (1 << 0)), %eax
    movl %eax, %cr0

    /* Load 64-bit GDT */
    lgdt gdt64_ptr

    /* Jump to 64-bit long mode */
    ljmp $0x08, $long_mode_start

.no_cpuid:
.no_long_mode:
    movb $'E', 0xB8000
    movb $0x4F, 0xB8001
.hang:
    hlt
    jmp .hang

.code64
long_mode_start:
    /* Reload data segment registers */
    movw $0x10, %ax
    movw %ax, %ds
    movw %ax, %es
    movw %ax, %fs
    movw %ax, %gs
    movw %ax, %ss

    /* Setup 64-bit stack */
    movq $stack_top, %rsp

    /* Pass multiboot parameters according to System V ABI */
    movl mb_magic(%rip), %esi
    movl mb_info(%rip), %edi

    /* Call C kernel main */
    call kernel_main

.halt64:
    cli
    hlt
    jmp .halt64

/* Boot GDT */
.section .rodata
.align 8
gdt64:
    .quad 0x0000000000000000 /* 0x00: Null Descriptor */
    .quad 0x00209A0000000000 /* 0x08: Kernel Code 64 (Present, Ring 0, Exec, Read, 64-bit) */
    .quad 0x0000920000000000 /* 0x10: Kernel Data 64 (Present, Ring 0, Read/Write) */
gdt64_end:

gdt64_ptr:
    .word gdt64_end - gdt64 - 1
    .quad gdt64

/* Early Page Tables and BSS */
.section .bss
.align 4096
boot_pml4:
    .skip 4096
boot_pdpt:
    .skip 4096
boot_pd:
    .skip 4096

.align 16
stack_bottom:
    .skip 32768 /* 32 KB early kernel stack */
stack_top:

mb_magic:
    .long 0
mb_info:
    .long 0
