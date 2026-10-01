ENTRY(_start)

SECTIONS {
    . = 1M;

    .boot : {
        KEEP(*(.multiboot_header))
        *(.boot)
    }

    .text : ALIGN(4K) {
        *(.text .text.*)
    }

    .rodata : ALIGN(4K) {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(4K) {
        *(.data .data.*)
    }

    .bss : ALIGN(4K) {
        *(COMMON)
        *(.bss .bss.*)
    }

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
    }

    kernel_end = .;
}
