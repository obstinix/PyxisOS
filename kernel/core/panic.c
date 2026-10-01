#include <pyxis/kernel.h>
#include <drivers/video/vga.h>

NORETURN void panic(const char *file, int line, const char *msg) {
    cli();

    vga_set_color(VGA_COLOR_WHITE, VGA_COLOR_RED);
    kprintf("\n");
    kprintf("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n");
    kprintf("                     KERNEL PANIC                           \n");
    kprintf("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n");
    kprintf("Location : %s:%d\n", file, line);
    kprintf("Reason   : %s\n", msg);
    kprintf("System   : %s v%s (%s)\n", PYXIS_OS_NAME, PYXIS_OS_VERSION, PYXIS_OS_CODENAME);
    kprintf("System halted. Please reboot hardware.\n");
    kprintf("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n");

    while (1) {
        hlt();
    }
}
