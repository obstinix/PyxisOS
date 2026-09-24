#ifndef PYXIS_MM_H
#define PYXIS_MM_H

#include <pyxis/types.h>

#define PAGE_SIZE 4096ULL
#define PAGE_SHIFT 12ULL
#define PAGE_MASK (~(PAGE_SIZE - 1))

#define PAGE_ALIGN_DOWN(addr) ((addr) & PAGE_MASK)
#define PAGE_ALIGN_UP(addr) (((addr) + PAGE_SIZE - 1) & PAGE_MASK)

#define KERNEL_VIRTUAL_BASE 0xFFFFFFFF80000000ULL

void pmm_init(phys_addr_t mem_size, phys_addr_t kernel_end);
phys_addr_t pmm_alloc_frame(void);
void pmm_free_frame(phys_addr_t frame);
size_t pmm_get_total_frames(void);
size_t pmm_get_used_frames(void);
size_t pmm_get_free_frames(void);

void vmm_init(void);
void vmm_map_page(uint64_t *pml4, virt_addr_t virt, phys_addr_t phys, uint64_t flags);
void vmm_unmap_page(uint64_t *pml4, virt_addr_t virt);
uint64_t *vmm_get_kernel_pml4(void);

void heap_init(void *start, size_t size);
void *kmalloc(size_t size);
void *kcalloc(size_t num, size_t size);
void kfree(void *ptr);
size_t heap_get_used(void);
size_t heap_get_total(void);

#endif
