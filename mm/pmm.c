#include <mm/pmm.h>
#include <pyxis/kernel.h>

#define PMM_MAX_FRAMES 1048576ULL /* Up to 4GB physical memory initially */
static uint64_t pmm_bitmap[PMM_MAX_FRAMES / 64];

static size_t total_frames = 0;
static size_t used_frames = 0;

static inline void set_bit(size_t bit) {
    pmm_bitmap[bit / 64] |= (1ULL << (bit % 64));
}

static inline void clear_bit(size_t bit) {
    pmm_bitmap[bit / 64] &= ~(1ULL << (bit % 64));
}

static inline bool test_bit(size_t bit) {
    return (pmm_bitmap[bit / 64] & (1ULL << (bit % 64))) != 0;
}

void pmm_init(phys_addr_t mem_size, phys_addr_t kernel_end) {
    total_frames = mem_size / PMM_FRAME_SIZE;
    if (total_frames > PMM_MAX_FRAMES) {
        total_frames = PMM_MAX_FRAMES;
    }

    /* Initially mark all frames as free */
    memset(pmm_bitmap, 0, sizeof(pmm_bitmap));
    used_frames = 0;

    /* Reserve first 1MB of physical memory (BIOS, VGA, real mode vectors) */
    size_t reserved_frames = 1048576 / PMM_FRAME_SIZE;
    for (size_t i = 0; i < reserved_frames; i++) {
        set_bit(i);
        used_frames++;
    }

    /* Reserve memory occupied by kernel image (up to kernel_end aligned) */
    size_t kernel_end_frame = (kernel_end + PMM_FRAME_SIZE - 1) / PMM_FRAME_SIZE;
    for (size_t i = reserved_frames; i < kernel_end_frame; i++) {
        set_bit(i);
        used_frames++;
    }

    kprintf("[PMM] Initialized: %d MB physical memory (%d frames, %d used, %d free)\n",
            (uint32_t)(mem_size / (1024 * 1024)),
            (uint32_t)total_frames,
            (uint32_t)used_frames,
            (uint32_t)(total_frames - used_frames));
}

phys_addr_t pmm_alloc_frame(void) {
    for (size_t i = 0; i < total_frames / 64; i++) {
        if (pmm_bitmap[i] != 0xFFFFFFFFFFFFFFFFULL) {
            for (size_t bit = 0; bit < 64; bit++) {
                if (!(pmm_bitmap[i] & (1ULL << bit))) {
                    size_t frame_index = i * 64 + bit;
                    if (frame_index >= total_frames) {
                        return 0;
                    }
                    set_bit(frame_index);
                    used_frames++;
                    return (phys_addr_t)(frame_index * PMM_FRAME_SIZE);
                }
            }
        }
    }
    return 0; /* Out of physical memory */
}

void pmm_free_frame(phys_addr_t frame) {
    size_t frame_index = frame / PMM_FRAME_SIZE;
    if (frame_index < total_frames && test_bit(frame_index)) {
        clear_bit(frame_index);
        if (used_frames > 0) {
            used_frames--;
        }
    }
}

size_t pmm_get_total_frames(void) {
    return total_frames;
}

size_t pmm_get_used_frames(void) {
    return used_frames;
}

size_t pmm_get_free_frames(void) {
    return total_frames - used_frames;
}
