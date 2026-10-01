#include <drivers/video/vga.h>
#include <pyxis/kernel.h>

static uint16_t *const vga_buffer = (uint16_t *)0xB8000;
static size_t cursor_x = 0;
static size_t cursor_y = 0;
static uint8_t current_color = 0x07; /* Light gray on black */

static inline uint16_t vga_entry(char c, uint8_t color) {
    return (uint16_t)c | ((uint16_t)color << 8);
}

static void update_cursor(size_t x, size_t y) {
    uint16_t pos = (uint16_t)(y * VGA_WIDTH + x);
    outb(0x3D4, 0x0F);
    outb(0x3D5, (uint8_t)(pos & 0xFF));
    outb(0x3D4, 0x0E);
    outb(0x3D5, (uint8_t)((pos >> 8) & 0xFF));
}

void vga_init(void) {
    cursor_x = 0;
    cursor_y = 0;
    current_color = (VGA_COLOR_BLACK << 4) | VGA_COLOR_LIGHT_GREY;
    vga_clear();
}

void vga_clear(void) {
    for (size_t y = 0; y < VGA_HEIGHT; y++) {
        for (size_t x = 0; x < VGA_WIDTH; x++) {
            vga_buffer[y * VGA_WIDTH + x] = vga_entry(' ', current_color);
        }
    }
    cursor_x = 0;
    cursor_y = 0;
    update_cursor(cursor_x, cursor_y);
}

void vga_set_color(uint8_t fg, uint8_t bg) {
    current_color = (bg << 4) | (fg & 0x0F);
}

static void scroll(void) {
    for (size_t y = 0; y < VGA_HEIGHT - 1; y++) {
        for (size_t x = 0; x < VGA_WIDTH; x++) {
            vga_buffer[y * VGA_WIDTH + x] = vga_buffer[(y + 1) * VGA_WIDTH + x];
        }
    }
    for (size_t x = 0; x < VGA_WIDTH; x++) {
        vga_buffer[(VGA_HEIGHT - 1) * VGA_WIDTH + x] = vga_entry(' ', current_color);
    }
    cursor_y = VGA_HEIGHT - 1;
}

void vga_putc(char c) {
    if (c == '\n') {
        cursor_x = 0;
        cursor_y++;
    } else if (c == '\r') {
        cursor_x = 0;
    } else if (c == '\t') {
        cursor_x = (cursor_x + 8) & ~7;
    } else if (c == '\b') {
        if (cursor_x > 0) {
            cursor_x--;
            vga_buffer[cursor_y * VGA_WIDTH + cursor_x] = vga_entry(' ', current_color);
        }
    } else if (c >= ' ') {
        vga_buffer[cursor_y * VGA_WIDTH + cursor_x] = vga_entry(c, current_color);
        cursor_x++;
    }

    if (cursor_x >= VGA_WIDTH) {
        cursor_x = 0;
        cursor_y++;
    }

    if (cursor_y >= VGA_HEIGHT) {
        scroll();
    }

    update_cursor(cursor_x, cursor_y);
}

void vga_puts(const char *str) {
    while (str && *str) {
        vga_putc(*str++);
    }
}
