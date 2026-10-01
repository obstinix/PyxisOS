#include <pyxis/fs.h>
#include <pyxis/kernel.h>
#include <mm/heap.h>

#define MAX_RAMFS_FILES 16

struct ramfs_file {
    char name[VFS_MAX_NAME];
    const void *data;
    size_t size;
    vfs_node_t node;
};

static struct ramfs_file ramfs_files[MAX_RAMFS_FILES];
static size_t ramfs_file_count = 0;
static vfs_node_t ramfs_root;

static ssize_t ramfs_file_read(vfs_node_t *node, uint64_t offset, size_t size, void *buffer) {
    struct ramfs_file *file = (struct ramfs_file *)node->priv;
    if (!file || offset >= file->size) return 0;

    size_t to_read = size;
    if (offset + to_read > file->size) {
        to_read = file->size - offset;
    }

    memcpy(buffer, (const uint8_t *)file->data + offset, to_read);
    return (ssize_t)to_read;
}

static vfs_node_t *ramfs_finddir(vfs_node_t *node, const char *name) {
    (void)node;
    for (size_t i = 0; i < ramfs_file_count; i++) {
        if (strcmp(ramfs_files[i].name, name) == 0) {
            return &ramfs_files[i].node;
        }
    }
    return NULL;
}

int ramfs_create_file(const char *name, const void *data, size_t size) {
    if (ramfs_file_count >= MAX_RAMFS_FILES) return -1;

    struct ramfs_file *f = &ramfs_files[ramfs_file_count++];
    strncpy(f->name, name, VFS_MAX_NAME - 1);
    f->data = data;
    f->size = size;

    memset(&f->node, 0, sizeof(vfs_node_t));
    strncpy(f->node.name, name, VFS_MAX_NAME - 1);
    f->node.flags = FS_FILE;
    f->node.length = size;
    f->node.read = ramfs_file_read;
    f->node.priv = f;

    return 0;
}

void ramfs_init(void) {
    memset(&ramfs_root, 0, sizeof(vfs_node_t));
    strcpy(ramfs_root.name, "/");
    ramfs_root.flags = FS_DIRECTORY;
    ramfs_root.finddir = ramfs_finddir;

    ramfs_file_count = 0;

    static const char *os_release_data = 
        "NAME=PyxisOS\n"
        "VERSION=0.1.0\n"
        "CODENAME=Lunar-Pyxis\n"
        "STAGE=Beta\n"
        "ARCH=x86_64\n";

    static const char *hostname_data = "pyxisos\n";
    static const char *readme_data = "PyxisOS Lunar Core System Initialized.\n";

    ramfs_create_file("os-release", os_release_data, strlen(os_release_data));
    ramfs_create_file("hostname", hostname_data, strlen(hostname_data));
    ramfs_create_file("README", readme_data, strlen(readme_data));

    vfs_mount("/", &ramfs_root);
    kprintf("[RAMFS] In-memory root filesystem mounted with %d initial files\n", (uint32_t)ramfs_file_count);
}
