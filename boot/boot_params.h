#ifndef PYXIS_BOOT_BOOT_PARAMS_H
#define PYXIS_BOOT_BOOT_PARAMS_H

#include <pyxis/types.h>

struct boot_params {
    uint32_t magic;
    uint64_t mem_lower_kb;
    uint64_t mem_upper_kb;
    uint64_t mmap_addr;
    uint32_t mmap_entries;
    uint64_t cmdline_addr;
    uint64_t initrd_addr;
    uint64_t initrd_size;
} PACKED;

#endif
