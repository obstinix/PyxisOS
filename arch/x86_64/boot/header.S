/* PyxisOS Multiboot Header */
.set MB_MAGIC,    0x1BADB002
.set MB_FLAGS,    (1 << 0) | (1 << 1)
.set MB_CHECKSUM, -(MB_MAGIC + MB_FLAGS)

.section .multiboot_header, "a"
.align 4
.long MB_MAGIC
.long MB_FLAGS
.long MB_CHECKSUM

/* Multiboot 2 Header */
.align 8
mb2_start:
.long 0xE85250D6
.long 0
.long mb2_end - mb2_start
.long -(0xE85250D6 + 0 + (mb2_end - mb2_start))

/* Multiboot 2 End Tag */
.short 0
.short 0
.long 8
mb2_end:
