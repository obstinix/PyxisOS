#ifndef PYXIS_MM_PMM_H
#define PYXIS_MM_PMM_H

#include <pyxis/types.h>

#define PMM_FRAME_SIZE 4096ULL

void pmm_init(phys_addr_t mem_size, phys_addr_t kernel_end);
phys_addr_t pmm_alloc_frame(void);
void pmm_free_frame(phys_addr_t frame);

size_t pmm_get_total_frames(void);
size_t pmm_get_used_frames(void);
size_t pmm_get_free_frames(void);

#endif
