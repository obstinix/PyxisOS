#ifndef PYXIS_MM_HEAP_H
#define PYXIS_MM_HEAP_H

#include <pyxis/types.h>

void heap_init(void *start, size_t size);
void *kmalloc(size_t size);
void *kcalloc(size_t num, size_t size);
void kfree(void *ptr);

size_t heap_get_used(void);
size_t heap_get_total(void);

#endif
