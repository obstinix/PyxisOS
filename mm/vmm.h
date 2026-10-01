#ifndef PYXIS_MM_VMM_H
#define PYXIS_MM_VMM_H

#include <pyxis/types.h>

#define PTE_PRESENT  (1ULL << 0)
#define PTE_WRITABLE (1ULL << 1)
#define PTE_USER     (1ULL << 2)
#define PTE_PWT      (1ULL << 3)
#define PTE_PCD      (1ULL << 4)
#define PTE_ACCESSED (1ULL << 5)
#define PTE_DIRTY    (1ULL << 6)
#define PTE_HUGE     (1ULL << 7)
#define PTE_GLOBAL   (1ULL << 8)
#define PTE_NX       (1ULL << 63)

#define VMM_PAGE_SIZE 4096ULL

void vmm_init(void);
void vmm_map_page(uint64_t *pml4, virt_addr_t virt, phys_addr_t phys, uint64_t flags);
void vmm_unmap_page(uint64_t *pml4, virt_addr_t virt);
uint64_t *vmm_get_kernel_pml4(void);
void vmm_switch_pml4(uint64_t *pml4);

#endif
