#include <pyxis/fs.h>
#include <pyxis/kernel.h>
#include <mm/heap.h>

static vfs_node_t *fs_root = NULL;

void vfs_init(void) {
    fs_root = NULL;
    kprintf("[VFS] Virtual File System initialized\n");
}

int vfs_mount(const char *path, vfs_node_t *node) {
    (void)path;
    if (!fs_root) {
        fs_root = node;
        return 0;
    }
    return -1;
}

vfs_node_t *vfs_open(const char *path, uint32_t flags) {
    if (!fs_root || !path) return NULL;

    if (path[0] == '/' && path[1] == '\0') {
        if (fs_root->open) fs_root->open(fs_root, flags);
        return fs_root;
    }

    const char *subpath = (path[0] == '/') ? path + 1 : path;
    if (fs_root->finddir) {
        vfs_node_t *node = fs_root->finddir(fs_root, subpath);
        if (node && node->open) {
            node->open(node, flags);
        }
        return node;
    }

    return NULL;
}

ssize_t vfs_read(vfs_node_t *node, uint64_t offset, size_t size, void *buffer) {
    if (node && node->read) {
        return node->read(node, offset, size, buffer);
    }
    return -1;
}

ssize_t vfs_write(vfs_node_t *node, uint64_t offset, size_t size, const void *buffer) {
    if (node && node->write) {
        return node->write(node, offset, size, buffer);
    }
    return -1;
}

int vfs_close(vfs_node_t *node) {
    if (node && node->close) {
        return node->close(node);
    }
    return 0;
}
