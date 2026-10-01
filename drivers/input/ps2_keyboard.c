#include <drivers/input/ps2_keyboard.h>
#include <arch/x86_64/interrupts/irq.h>
#include <arch/x86_64/interrupts/pic.h>
#include <pyxis/kernel.h>

#define KEYBOARD_DATA_PORT   0x60
#define KEYBOARD_STATUS_PORT 0x64
#define KEYBOARD_BUFFER_SIZE 128

static char key_buffer[KEYBOARD_BUFFER_SIZE];
static volatile size_t buf_read_idx = 0;
static volatile size_t buf_write_idx = 0;
static bool shift_pressed = false;

static const char scancode_ascii_nomod[128] = {
    0,   27,  '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\b',
    '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',
    0,   'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`',
    0,   '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', 0,
    '*', 0,   ' '
};

static const char scancode_ascii_shift[128] = {
    0,   27,  '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '\b',
    '\t', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '{', '}', '\n',
    0,   'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ':', '"', '~',
    0,   '|', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', '<', '>', '?', 0,
    '*', 0,   ' '
};

static void keyboard_irq_handler(struct interrupt_frame *frame) {
    (void)frame;
    uint8_t scancode = inb(KEYBOARD_DATA_PORT);

    if (scancode == 0x2A || scancode == 0x36) {
        shift_pressed = true;
        return;
    }
    if (scancode == 0xAA || scancode == 0xB6) {
        shift_pressed = false;
        return;
    }

    /* Key release */
    if (scancode & 0x80) {
        return;
    }

    char ch = shift_pressed ? scancode_ascii_shift[scancode] : scancode_ascii_nomod[scancode];
    if (ch != 0) {
        size_t next_write = (buf_write_idx + 1) % KEYBOARD_BUFFER_SIZE;
        if (next_write != buf_read_idx) {
            key_buffer[buf_write_idx] = ch;
            buf_write_idx = next_write;
        }
    }
}

void keyboard_init(void) {
    buf_read_idx = 0;
    buf_write_idx = 0;
    shift_pressed = false;

    register_interrupt_handler(33, keyboard_irq_handler);
    pic_clear_mask(1); /* Enable IRQ1 on master PIC */

    kprintf("[KEYBOARD] PS/2 Keyboard initialized on IRQ1\n");
}

char keyboard_getchar_nonblocking(void) {
    if (buf_read_idx == buf_write_idx) {
        return 0;
    }
    char ch = key_buffer[buf_read_idx];
    buf_read_idx = (buf_read_idx + 1) % KEYBOARD_BUFFER_SIZE;
    return ch;
}

char keyboard_getchar(void) {
    char c = 0;
    while ((c = keyboard_getchar_nonblocking()) == 0) {
        hlt();
    }
    return c;
}
