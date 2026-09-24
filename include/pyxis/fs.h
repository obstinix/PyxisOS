#ifndef PYXIS_FS_H
#define PYXIS_FS_H

#include <pyxis/types.h>

#define FS_FILE        0x01
#define FS_DIRECTORY   0x02
#define FS_CHARDEVICE  0x03
#define FS_BLOCKDEVICE 0x04

#define VFS_MAX_PATH 256
#define VFS_MAX_NAME 64

struct vfs_node;

typedef ssize_t (*vfs_read_fn)(struct vfs_node *node, uint64_t offset, size_t size, void *buffer);
typedef ssize_t (*vfs_write_fn)(struct vfs_node *node, uint64_t offset, size_t size, const void *buffer);
typedef int (*vfs_open_fn)(struct vfs_node *node, uint32_t flags);
typedef int (*vfs_close_fn)(struct vfs_node *node);
typedef struct vfs_node *(*vfs_finddir_fn)(struct vfs_node *node, const char *name);

typedef struct vfs_node {
    char name[VFS_MAX_NAME];
    uint32_t flags;
    uint32_t inode;
    uint64_t length;
    vfs_read_fn read;
    vfs_write_fn write;
    vfs_open_fn open;
    vfs_close_fn close;
    vfs_finddir_fn finddir;
    void *priv;
} vfs_node_t;

void vfs_init(void);
int vfs_mount(const char *path, vfs_node_t *node);
vfs_node_t *vfs_open(const char *path, uint32_t flags);
ssize_t vfs_read(vfs_node_t *node, uint64_t offset, size_t size, void *buffer);
ssize_t vfs_write(vfs_node_t *node, uint64_t offset, size_t size, const void *buffer);
int vfs_close(vfs_node_t *node);

void ramfs_init(void);
int ramfs_create_file(const char *name, const void *data, size_t size);

#endif
