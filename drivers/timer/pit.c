#include <drivers/timer/pit.h>
#include <arch/x86_64/interrupts/irq.h>
#include <arch/x86_64/interrupts/pic.h>
#include <pyxis/kernel.h>

#define PIT_BASE_FREQUENCY 1193182ULL
#define PIT_CHANNEL0_DATA  0x40
#define PIT_COMMAND_PORT   0x43

static volatile uint64_t pit_ticks = 0;
static uint32_t current_frequency = 100;

static void pit_irq_handler(struct interrupt_frame *frame) {
    (void)frame;
    pit_ticks++;
}

void pit_init(uint32_t frequency_hz) {
    if (frequency_hz == 0) frequency_hz = 100;
    current_frequency = frequency_hz;

    uint16_t divisor = (uint16_t)(PIT_BASE_FREQUENCY / frequency_hz);

    /* Command 0x36: Channel 0, Access mode lo/hi, Mode 3 (Square Wave), Binary */
    outb(PIT_COMMAND_PORT, 0x36);
    outb(PIT_CHANNEL0_DATA, (uint8_t)(divisor & 0xFF));
    outb(PIT_CHANNEL0_DATA, (uint8_t)((divisor >> 8) & 0xFF));

    register_interrupt_handler(32, pit_irq_handler);
    pic_clear_mask(0); /* Enable IRQ0 on master PIC */

    kprintf("[PIT] Initialized timer at %d Hz\n", frequency_hz);
}

uint64_t pit_get_ticks(void) {
    return pit_ticks;
}

uint64_t pit_get_uptime_seconds(void) {
    return pit_ticks / current_frequency;
}

void pit_sleep_ms(uint32_t ms) {
    uint64_t target_ticks = pit_ticks + ((uint64_t)ms * current_frequency) / 1000;
    while (pit_ticks < target_ticks) {
        hlt();
    }
}
