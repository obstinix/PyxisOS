#ifndef PYXIS_TYPES_H
#define PYXIS_TYPES_H

typedef signed char int8_t;
typedef unsigned char uint8_t;
typedef signed short int16_t;
typedef unsigned short uint16_t;
typedef signed int int32_t;
typedef unsigned int uint32_t;
typedef signed long long int64_t;
typedef unsigned long long uint64_t;

typedef uint64_t size_t;
typedef int64_t ssize_t;
typedef uint64_t uintptr_t;
typedef int64_t intptr_t;

typedef uint64_t phys_addr_t;
typedef uint64_t virt_addr_t;

#define NULL ((void *)0)

#ifndef __cplusplus
typedef enum {
    false = 0,
    true = 1
} bool;
#endif

#define PACKED __attribute__((packed))
#define ALIGNED(n) __attribute__((aligned(n)))
#define NORETURN __attribute__((noreturn))

#endif
