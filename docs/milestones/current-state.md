# PyxisOS Current Operating System State

This document records the exact physical and software specifications of the active PyxisOS reference system.

---

## 1. Operating System Metadata

- **Name:** PyxisOS
- **Version:** 0.1.0
- **Codename:** Lunar-Pyxis
- **Variant:** Beta
- **Build ID:** rolling
- **Hostname:** `pyxisos`
- **Architecture:** x86_64
- **Color Accent:** `38;2;220;40;80` (Deep Crimson Red)

---

## 2. Hardware Specification

- **Platform:** Lenovo ThinkPad X1 Carbon 5th Gen
- **Processor:** Intel Core i7-7600U (4 threads @ 2.80GHz base, up to 3.90GHz turbo)
- **Memory:** 16 GB LPDDR3 RAM
- **Storage:** NVMe Solid-State Drive (`/dev/nvme0n1`)
- **Display:** 14" Full HD (1920x1080) / WQHD IPS panel
- **Graphics:** Intel HD Graphics 620

---

## 3. Storage & Partition Layout

| Device | Mount Point | UUID | Type | Description |
|---|---|---|---|---|
| `/dev/nvme0n1p2` | *Unmounted / Win EFI* | `9A05-B7D2` | `vfat` | Windows 10 EFI System Partition |
| `/dev/nvme0n1p6` | `/boot` | `76B7-515F` | `vfat` | PyxisOS EFI Boot Partition |
| `/dev/nvme0n1p7` | `/` | `00955744-524c-403a-99f2-69818444cd30` | `ext4` | PyxisOS Root Filesystem |

---

## 4. Boot Configuration

### `/boot/loader/loader.conf`
```ini
default pyxisos
timeout 5
editor no
```

### `/boot/loader/entries/pyxisos.conf`
```ini
title  PyxisOS
linux  /vmlinuz-linux
initrd /intel-ucode.img
initrd /initramfs-linux.img
options root=UUID=00955744-524c-403a-99f2-69818444cd30
```

### `/boot/loader/entries/windows.conf`
```ini
title Windows 10
efi /EFI/Microsoft/Boot/bootmgfw.efi
```

---

## 5. Subsystem Diagnostics (`pyxis doctor`)

```text
============================================================
                    PYXISOS DOCTOR
============================================================

01  IDENTITY
    [ OK ] PyxisOS identity
    [ OK ] Version 0.1.0
    [ OK ] Release Lunar-Pyxis
    [ OK ] Hostname: pyxisos

02  BOOT
    [ OK ] /boot mounted
    [WARN] systemd-boot installation state
    [ OK ] PyxisOS loader entry

03  KERNEL
    [ OK ] Linux kernel
    [ OK ] Linux initramfs
    [ OK ] Intel microcode

04  SERVICES
    [ OK ] SDDM active
    [ OK ] SDDM enabled
    [ OK ] NetworkManager active

05  DESKTOP
    [ OK ] KDE Plasma available
    [WARN] Hyprland not installed

06  STORAGE
    [ OK ] Root filesystem mounted
    [ OK ] Boot filesystem mounted

------------------------------------------------------------
RESULT :: NO CRITICAL PROBLEMS DETECTED
------------------------------------------------------------
```

### Diagnostic Notes:
- **systemd-boot [WARN]:** The installation state warning is non-critical. UEFI entries and systemd-boot 261.2 correctly execute the active `pyxisos.conf` entry.
- **Hyprland [WARN]:** Hyprland is intentionally not installed yet to prioritize KDE Plasma stability.
