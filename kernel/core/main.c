#include <pyxis/types.h>
#include <pyxis/kernel.h>
#include <pyxis/mm.h>
#include <pyxis/drivers.h>
#include <pyxis/fs.h>
#include <boot/multiboot.h>
#include <arch/x86_64/cpu/gdt.h>
#include <arch/x86_64/cpu/idt.h>
#include <arch/x86_64/interrupts/pic.h>
#include <arch/x86_64/interrupts/irq.h>
#include <drivers/char/serial.h>
#include <drivers/video/vga.h>
#include <drivers/timer/pit.h>
#include <drivers/input/ps2_keyboard.h>
#include <userspace/shell/shell.h>

extern uint8_t kernel_end;
extern void rust_kernel_init(void);

static void print_banner(void) {
    vga_set_color(VGA_COLOR_LIGHT_RED, VGA_COLOR_BLACK);
    kprintf("\n");
    kprintf("    ____             _      ____  _____\n");
    kprintf("   / __ \\__  ___  __(_)____/ __ \\/ ___/\n");
    kprintf("  / /_/ / / / / |/_/ / ___/ / / /\\__ \\ \n");
    kprintf(" / ____/ /_/ />  </ (__  ) /_/ /___/ / \n");
    kprintf("/_/    \\__, /_/|_/_/____/\\____//____/  \n");
    kprintf("      /____/                           \n");
    vga_set_color(VGA_COLOR_WHITE, VGA_COLOR_BLACK);
    kprintf("============================================================\n");
    kprintf("  %s %s // %s (Release: %s, Arch: %s)\n", 
            PYXIS_OS_NAME, PYXIS_OS_VERSION, PYXIS_OS_STAGE, PYXIS_OS_CODENAME, PYXIS_OS_ARCH);
    kprintf("  Hybrid Low-Level x86_64 Operating System Architecture\n");
    kprintf("============================================================\n\n");
}

void kernel_main(uint32_t mb_info_addr, uint32_t mb_magic) {
    /* 1. Early debug console */
    serial_init();
    serial_puts("\n[PYXISOS] Early serial logging initialized.\n");

    /* 2. Visual console */
    vga_init();
    print_banner();

    /* 3. Core CPU structures */
    kprintf("[BOOT] Initializing Global Descriptor Table & TSS...\n");
    gdt_init();

    kprintf("[BOOT] Initializing Interrupt Descriptor Table & Exception Vectors...\n");
    idt_init();
    irq_init();

    kprintf("[BOOT] Programming 8259 Legacy PIC (remap to vectors 32..47)...\n");
    pic_init();

    /* 4. Memory subsystem */
    phys_addr_t total_memory = 128ULL * 1024ULL * 1024ULL; /* Default fallback 128 MB */
    if (mb_magic == MULTIBOOT_BOOTLOADER_MAGIC && mb_info_addr != 0) {
        struct multiboot_info *mbi = (struct multiboot_info *)(uint64_t)mb_info_addr;
        if (mbi->flags & (1 << 0)) {
            total_memory = (phys_addr_t)(mbi->mem_upper + 1024) * 1024ULL;
        }
    }

    kprintf("[BOOT] Initializing Physical Memory Manager...\n");
    pmm_init(total_memory, (phys_addr_t)&kernel_end);

    kprintf("[BOOT] Initializing 4-level Virtual Memory Manager (Paging)...\n");
    vmm_init();

    /* Initialize 16MB kernel heap starting after lower identity mappings at 32MB */
    kprintf("[BOOT] Initializing Kernel Heap Allocator...\n");
    heap_init((void *)0x2000000ULL, 16ULL * 1024ULL * 1024ULL);

    /* 5. Device drivers */
    kprintf("[BOOT] Initializing 8254 Programmable Interval Timer (100 Hz)...\n");
    pit_init(100);

    kprintf("[BOOT] Initializing PS/2 Keyboard Driver on IRQ1...\n");
    keyboard_init();

    /* 6. Filesystem */
    kprintf("[BOOT] Mounting Virtual File System & RamFS...\n");
    vfs_init();
    ramfs_init();

    /* 7. Rust subsystems */
    kprintf("[BOOT] Initializing Rust Kernel Core (Scheduler, Tasks, IPC)...\n");
    rust_kernel_init();

    /* 8. Enable interrupts and launch user interface */
    kprintf("[BOOT] Enabling CPU Interrupts (sti)...\n");
    sti();

    kprintf("[BOOT] System initialization complete. Entering Pyxis Shell.\n");
    shell_init();
    shell_run();

    /* System halt if shell exits */
    cli();
    while (1) {
        hlt();
    }
}
