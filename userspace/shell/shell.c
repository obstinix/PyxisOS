#include <userspace/shell/shell.h>
#include <drivers/video/vga.h>
#include <drivers/input/ps2_keyboard.h>
#include <drivers/timer/pit.h>
#include <mm/pmm.h>
#include <mm/heap.h>
#include <pyxis/fs.h>
#include <pyxis/kernel.h>

#define CMD_BUFFER_SIZE 128

static void cmd_help(void) {
    kprintf("\nAvailable Commands:\n");
    kprintf("  help     - Show available commands\n");
    kprintf("  info     - Display PyxisOS identity and build version\n");
    kprintf("  mem      - Show physical frame and kernel heap memory metrics\n");
    kprintf("  uptime   - Show elapsed system uptime\n");
    kprintf("  cat      - Read a file from root ramfs (e.g. 'cat os-release')\n");
    kprintf("  clear    - Clear screen buffer\n");
    kprintf("  reboot   - Reboot system hardware\n");
    kprintf("\n");
}

static void cmd_info(void) {
    kprintf("\n============================================================\n");
    kprintf("                     PYXISOS IDENTITY                       \n");
    kprintf("============================================================\n");
    kprintf("  Operating System : %s\n", PYXIS_OS_NAME);
    kprintf("  Version          : %s\n", PYXIS_OS_VERSION);
    kprintf("  Codename         : %s\n", PYXIS_OS_CODENAME);
    kprintf("  Stage            : %s\n", PYXIS_OS_STAGE);
    kprintf("  Architecture     : %s\n", PYXIS_OS_ARCH);
    kprintf("  Kernel Layer     : Hybrid C / Rust Core with ASM Bootstrap\n");
    kprintf("============================================================\n\n");
}

static void cmd_mem(void) {
    kprintf("\nMemory Metrics:\n");
    kprintf("  Physical Frames Total : %d\n", (uint32_t)pmm_get_total_frames());
    kprintf("  Physical Frames Used  : %d (%d KB)\n", 
            (uint32_t)pmm_get_used_frames(), 
            (uint32_t)((pmm_get_used_frames() * 4096) / 1024));
    kprintf("  Physical Frames Free  : %d (%d KB)\n", 
            (uint32_t)pmm_get_free_frames(), 
            (uint32_t)((pmm_get_free_frames() * 4096) / 1024));
    kprintf("  Kernel Heap Used      : %d bytes\n", (uint32_t)heap_get_used());
    kprintf("  Kernel Heap Total     : %d bytes\n\n", (uint32_t)heap_get_total());
}

static void cmd_uptime(void) {
    uint64_t sec = pit_get_uptime_seconds();
    uint64_t ticks = pit_get_ticks();
    kprintf("Uptime: %d seconds (%d timer ticks)\n", (uint32_t)sec, (uint32_t)ticks);
}

static void cmd_cat(const char *filename) {
    while (*filename == ' ') filename++;
    if (*filename == '\0') {
        kprintf("Usage: cat <filename>\n");
        return;
    }

    vfs_node_t *file = vfs_open(filename, 0);
    if (!file) {
        kprintf("cat: '%s': File not found\n", filename);
        return;
    }

    char buf[128];
    ssize_t bytes;
    uint64_t offset = 0;
    while ((bytes = vfs_read(file, offset, sizeof(buf) - 1, buf)) > 0) {
        buf[bytes] = '\0';
        kprintf("%s", buf);
        offset += bytes;
    }
    vfs_close(file);
}

static void cmd_reboot(void) {
    kprintf("Rebooting PyxisOS...\n");
    pit_sleep_ms(200);

    /* Pulse reset line using 8042 keyboard controller */
    uint8_t good = 0x02;
    while (good & 0x02) {
        good = inb(0x64);
    }
    outb(0x64, 0xFE);

    /* Fallback triple-fault */
    cli();
    struct {
        uint16_t limit;
        uint64_t base;
    } PACKED null_idt = {0, 0};
    __asm__ volatile ("lidt (%0); int3" : : "r"(&null_idt));
    while (1) { hlt(); }
}

void shell_init(void) {
    kprintf("[SHELL] Interactive kernel terminal shell initialized\n");
}

void shell_run(void) {
    char cmd[CMD_BUFFER_SIZE];
    size_t pos = 0;

    kprintf("\nType 'help' for commands list.\n");
    kprintf("pyxisos> ");

    while (1) {
        char c = keyboard_getchar();
        if (c == '\r' || c == '\n') {
            kprintf("\n");
            cmd[pos] = '\0';

            if (strcmp(cmd, "help") == 0) {
                cmd_help();
            } else if (strcmp(cmd, "info") == 0) {
                cmd_info();
            } else if (strcmp(cmd, "mem") == 0) {
                cmd_mem();
            } else if (strcmp(cmd, "uptime") == 0) {
                cmd_uptime();
            } else if (strncmp(cmd, "cat ", 4) == 0) {
                cmd_cat(cmd + 4);
            } else if (strcmp(cmd, "clear") == 0) {
                vga_clear();
            } else if (strcmp(cmd, "reboot") == 0) {
                cmd_reboot();
            } else if (pos > 0) {
                kprintf("Unknown command: '%s'. Type 'help' for available commands.\n", cmd);
            }

            pos = 0;
            kprintf("pyxisos> ");
        } else if (c == '\b') {
            if (pos > 0) {
                pos--;
                kputc('\b');
            }
        } else if (c >= ' ' && pos < CMD_BUFFER_SIZE - 1) {
            cmd[pos++] = c;
            kputc(c);
        }
    }
}
