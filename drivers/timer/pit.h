#ifndef PYXIS_DRIVERS_PIT_H
#define PYXIS_DRIVERS_PIT_H

#include <pyxis/types.h>

void pit_init(uint32_t frequency_hz);
uint64_t pit_get_ticks(void);
uint64_t pit_get_uptime_seconds(void);
void pit_sleep_ms(uint32_t ms);

#endif
