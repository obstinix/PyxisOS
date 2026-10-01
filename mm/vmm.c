#include <mm/vmm.h>
#include <mm/pmm.h>
#include <pyxis/kernel.h>

static uint64_t *kernel_pml4 = NULL;

static inline size_t pml4_index(virt_addr_t virt) {
    return (virt >> 39) & 0x1FF;
}

static inline size_t pdpt_index(virt_addr_t virt) {
    return (virt >> 30) & 0x1FF;
}

static inline size_t pd_index(virt_addr_t virt) {
    return (virt >> 21) & 0x1FF;
}

static inline size_t pt_index(virt_addr_t virt) {
    return (virt >> 12) & 0x1FF;
}

static inline void invlpg(virt_addr_t addr) {
    __asm__ volatile ("invlpg (%0)" : : "r"(addr) : "memory");
}

void vmm_init(void) {
    /* Allocate PML4 from PMM */
    phys_addr_t pml4_phys = pmm_alloc_frame();
    KASSERT(pml4_phys != 0);

    kernel_pml4 = (uint64_t *)pml4_phys;
    memset(kernel_pml4, 0, VMM_PAGE_SIZE);

    /* Identity map first 64MB of physical memory */
    for (virt_addr_t addr = 0; addr < 64 * 1024 * 1024; addr += VMM_PAGE_SIZE) {
        vmm_map_page(kernel_pml4, addr, (phys_addr_t)addr, PTE_PRESENT | PTE_WRITABLE);
    }

    vmm_switch_pml4(kernel_pml4);
    kprintf("[VMM] Paging initialized (4-level x86_64 PML4: 0x%p)\n", kernel_pml4);
}

void vmm_map_page(uint64_t *pml4, virt_addr_t virt, phys_addr_t phys, uint64_t flags) {
    size_t pml4_i = pml4_index(virt);
    size_t pdpt_i = pdpt_index(virt);
    size_t pd_i   = pd_index(virt);
    size_t pt_i   = pt_index(virt);

    uint64_t *pdpt;
    if (!(pml4[pml4_i] & PTE_PRESENT)) {
        phys_addr_t frame = pmm_alloc_frame();
        KASSERT(frame != 0);
        pdpt = (uint64_t *)frame;
        memset(pdpt, 0, VMM_PAGE_SIZE);
        pml4[pml4_i] = frame | PTE_PRESENT | PTE_WRITABLE | (flags & PTE_USER);
    } else {
        pdpt = (uint64_t *)(pml4[pml4_i] & ~0xFFFULL);
    }

    uint64_t *pd;
    if (!(pdpt[pdpt_i] & PTE_PRESENT)) {
        phys_addr_t frame = pmm_alloc_frame();
        KASSERT(frame != 0);
        pd = (uint64_t *)frame;
        memset(pd, 0, VMM_PAGE_SIZE);
        pdpt[pdpt_i] = frame | PTE_PRESENT | PTE_WRITABLE | (flags & PTE_USER);
    } else {
        pd = (uint64_t *)(pdpt[pdpt_i] & ~0xFFFULL);
    }

    uint64_t *pt;
    if (!(pd[pd_i] & PTE_PRESENT)) {
        phys_addr_t frame = pmm_alloc_frame();
        KASSERT(frame != 0);
        pt = (uint64_t *)frame;
        memset(pt, 0, VMM_PAGE_SIZE);
        pd[pd_i] = frame | PTE_PRESENT | PTE_WRITABLE | (flags & PTE_USER);
    } else {
        pt = (uint64_t *)(pd[pd_i] & ~0xFFFULL);
    }

    pt[pt_i] = (phys & ~0xFFFULL) | flags | PTE_PRESENT;
    invlpg(virt);
}

void vmm_unmap_page(uint64_t *pml4, virt_addr_t virt) {
    size_t pml4_i = pml4_index(virt);
    size_t pdpt_i = pdpt_index(virt);
    size_t pd_i   = pd_index(virt);
    size_t pt_i   = pt_index(virt);

    if (!(pml4[pml4_i] & PTE_PRESENT)) return;
    uint64_t *pdpt = (uint64_t *)(pml4[pml4_i] & ~0xFFFULL);

    if (!(pdpt[pdpt_i] & PTE_PRESENT)) return;
    uint64_t *pd = (uint64_t *)(pdpt[pdpt_i] & ~0xFFFULL);

    if (!(pd[pd_i] & PTE_PRESENT)) return;
    uint64_t *pt = (uint64_t *)(pd[pd_i] & ~0xFFFULL);

    pt[pt_i] = 0;
    invlpg(virt);
}

uint64_t *vmm_get_kernel_pml4(void) {
    return kernel_pml4;
}

void vmm_switch_pml4(uint64_t *pml4) {
    __asm__ volatile ("mov %0, %%cr3" : : "r"(pml4) : "memory");
}
