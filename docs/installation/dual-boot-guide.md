# UEFI Dual-Boot Guide: PyxisOS & Windows 10

This guide details how PyxisOS and Windows 10 coexist cleanly on the same NVMe drive using systemd-boot without risk to Windows partitions.

---

## Partition Layout

On the reference system (`/dev/nvme0n1`), the disk is partitioned as follows:

```text
/dev/nvme0n1
├── p1: Windows Recovery Environment
├── p2: Windows EFI System Partition (ESP) [UUID: 9A05-B7D2]
├── p3: Microsoft Reserved (MSR)
├── p4: Windows 10 OS (BitLocker / NTFS)
├── p5: OEM / Lenovo Recovery
├── p6: PyxisOS EFI / Boot Partition (/boot, vfat) [UUID: 76B7-515F]
└── p7: PyxisOS Root Partition (/, ext4) [UUID: 00955744-524c-403a-99f2-69818444cd30]
```

---

## Rules of Engagement

1. **Do not format or resize partitions p1 through p5.**
2. **Do not mount `/dev/nvme0n1p6` directly to `/mnt`.** Mount `/dev/nvme0n1p7` to `/mnt` first, then mount `/dev/nvme0n1p6` to `/mnt/boot`.
3. **Do not repartition the disk.** Any partition table changes risk invalidating Windows partition table offsets.

---

## systemd-boot Configuration

PyxisOS uses `systemd-boot` located in `/dev/nvme0n1p6` (`/boot`).

### Main Config: `/boot/loader/loader.conf`
```ini
default pyxisos
timeout 5
editor no
```

### PyxisOS Entry: `/boot/loader/entries/pyxisos.conf`
```ini
title  PyxisOS
linux  /vmlinuz-linux
initrd /intel-ucode.img
initrd /initramfs-linux.img
options root=UUID=00955744-524c-403a-99f2-69818444cd30 rw quiet
```

### Windows Entry: `/boot/loader/entries/windows.conf`
```ini
title Windows 10
efi /EFI/Microsoft/Boot/bootmgfw.efi
```

*Note:* If Windows boot files reside on the Windows EFI partition (`/dev/nvme0n1p2`), systemd-boot can either chainload across ESPs if auto-detected by UEFI or files can be bridged into the PyxisOS ESP under `/EFI/Microsoft/Boot/bootmgfw.efi`.
