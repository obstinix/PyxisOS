#!/usr/bin/env bash
# PyxisOS Boot Debug / Recovery Command Reference
# Run section-by-section. DO NOT execute this entire file automatically.
# Root: /dev/nvme0n1p7
# PyxisOS EFI/boot: /dev/nvme0n1p6
# Windows EFI: /dev/nvme0n1p2
# Root UUID: 00955744-524c-403a-9912-69818444cd30

set -u

cat <<'EOF'
============================================================
PyxisOS BOOT DEBUG / RECOVERY COMMAND REFERENCE
============================================================

SECTION 1 — Verify UEFI
------------------------------------------------------------
ls /sys/firmware/efi

SECTION 2 — Verify disks
------------------------------------------------------------
lsblk -f
ls /dev/nvme*

SECTION 3 — Check root filesystem
------------------------------------------------------------
# p7 MUST be unmounted before fsck.
umount /mnt/boot 2>/dev/null || true
umount /mnt 2>/dev/null || true
fsck -f /dev/nvme0n1p7

SECTION 4 — Correct mounting
------------------------------------------------------------
mount /dev/nvme0n1p7 /mnt
mount /dev/nvme0n1p6 /mnt/boot
ls /mnt
ls /mnt/boot

# NEVER mount p6 directly on /mnt.

SECTION 5 — Verify root UUID
------------------------------------------------------------
blkid /dev/nvme0n1p7

Expected:
UUID="00955744-524c-403a-9912-69818444cd30"

SECTION 6 — Verify systemd-boot configuration
------------------------------------------------------------
cat /mnt/boot/loader/loader.conf
cat /mnt/boot/loader/entries/pyxisos.conf

Expected root option:
root=UUID=00955744-524c-403a-9912-69818444cd30

SECTION 7 — Verify Windows EFI
------------------------------------------------------------
ls -l /mnt/boot/EFI/Microsoft/Boot/bootmgfw.efi
cat /mnt/boot/loader/entries/windows.conf

Expected:
title Windows 10
efi /EFI/Microsoft/Boot/bootmgfw.efi

# Correct filename is bootmgfw.efi, NOT bootmgw.efi.

SECTION 8 — Verify systemd-boot files
------------------------------------------------------------
ls -l /mnt/boot/EFI/systemd/systemd-bootx64.efi
ls -l /mnt/boot/EFI/BOOT/BOOTX64.EFI

SECTION 9 — Enter installed system
------------------------------------------------------------
arch-chroot /mnt

SECTION 10 — NEXT INVESTIGATION
------------------------------------------------------------
# This is the important next step. Inspect before rebuilding.

cat /etc/mkinitcpio.conf
cat /etc/fstab
ls -l /boot
ls -l /boot/loader
ls -l /boot/loader/entries

grep -n '00955744' /etc/fstab

# Inspect initramfs for storage/filesystem support:
lsinitcpio -a /boot/initramfs-linux.img | grep -Ei 'nvme|ext4|filesystem|block'

SECTION 11 — Rebuild only after inspection
------------------------------------------------------------
mkinitcpio -P

Then:
ls -lh /boot/initramfs-linux.img
ls -lh /boot/vmlinuz-linux

SECTION 12 — Verify systemd-boot
------------------------------------------------------------
bootctl status
bootctl list

SECTION 13 — EFI entries
------------------------------------------------------------
# Run with EFI variables available:
efibootmgr -v

# Do NOT keep creating duplicate PyxisOS entries.
# systemd-boot already launches the kernel.

SECTION 14 — Reboot
------------------------------------------------------------
exit
umount /mnt/boot
umount /mnt
reboot

============================================================
CURRENT DIAGNOSIS
============================================================

UEFI                 WORKING
systemd-boot         WORKING
kernel               STARTS
initramfs            STARTS
NVMe from ISO        WORKING
root filesystem      PASSES fsck
root UUID             CORRECT
Windows EFI          PRESENT
root handoff         FAILING

Current failure:
Timed out waiting for
/dev/disk/by-uuid/00955744-524c-403a-9912-69818444cd30

NEXT FOCUS:
Early-boot NVMe / root-device discovery.

DO NOT:
- repartition
- reinstall
- repeatedly change systemd-boot
- repeatedly create efibootmgr entries
- modify Windows unnecessarily
- blindly repeat mkinitcpio without inspecting configuration

============================================================
END
============================================================
EOF
