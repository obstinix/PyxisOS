#include <mm/heap.h>
#include <pyxis/kernel.h>

struct heap_block {
    size_t size;
    bool is_free;
    struct heap_block *next;
} PACKED;

#define BLOCK_HEADER_SIZE sizeof(struct heap_block)
#define HEAP_ALIGNMENT 16ULL
#define ALIGN_UP(val) (((val) + (HEAP_ALIGNMENT - 1)) & ~(HEAP_ALIGNMENT - 1))

static struct heap_block *heap_head = NULL;
static size_t heap_total_size = 0;
static size_t heap_used_size = 0;

void heap_init(void *start, size_t size) {
    KASSERT(start != NULL && size > BLOCK_HEADER_SIZE);

    heap_head = (struct heap_block *)start;
    heap_head->size = size - BLOCK_HEADER_SIZE;
    heap_head->is_free = true;
    heap_head->next = NULL;

    heap_total_size = size;
    heap_used_size = 0;

    kprintf("[HEAP] Initialized kernel heap at 0x%p (%d KB)\n", start, (uint32_t)(size / 1024));
}

void *kmalloc(size_t size) {
    if (size == 0) return NULL;

    size = ALIGN_UP(size);
    struct heap_block *curr = heap_head;

    while (curr) {
        if (curr->is_free && curr->size >= size) {
            /* Can we split this block? */
            if (curr->size >= size + BLOCK_HEADER_SIZE + HEAP_ALIGNMENT) {
                struct heap_block *new_block = (struct heap_block *)((uint8_t *)curr + BLOCK_HEADER_SIZE + size);
                new_block->size = curr->size - size - BLOCK_HEADER_SIZE;
                new_block->is_free = true;
                new_block->next = curr->next;

                curr->size = size;
                curr->next = new_block;
            }
            curr->is_free = false;
            heap_used_size += curr->size + BLOCK_HEADER_SIZE;
            return (void *)((uint8_t *)curr + BLOCK_HEADER_SIZE);
        }
        curr = curr->next;
    }

    kprintf("[HEAP] Out of memory allocating %d bytes\n", (uint32_t)size);
    return NULL;
}

void *kcalloc(size_t num, size_t size) {
    size_t total = num * size;
    void *ptr = kmalloc(total);
    if (ptr) {
        memset(ptr, 0, total);
    }
    return ptr;
}

void kfree(void *ptr) {
    if (!ptr) return;

    struct heap_block *block = (struct heap_block *)((uint8_t *)ptr - BLOCK_HEADER_SIZE);
    block->is_free = true;
    heap_used_size -= (block->size + BLOCK_HEADER_SIZE);

    /* Coalesce adjacent free blocks */
    struct heap_block *curr = heap_head;
    while (curr && curr->next) {
        if (curr->is_free && curr->next->is_free) {
            curr->size += curr->next->size + BLOCK_HEADER_SIZE;
            curr->next = curr->next->next;
        } else {
            curr = curr->next;
        }
    }
}

size_t heap_get_used(void) {
    return heap_used_size;
}

size_t heap_get_total(void) {
    return heap_total_size;
}
