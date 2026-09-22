# PyxisOS Boot Configuration

This directory contains boot configuration references, recovery scripts, and templates for systemd-boot.

## Files

- `pyxisos_boot_commands.sh`: Diagnostic and recovery procedure for early NVMe root discovery and initramfs verification.
- `loader.conf.example`: Reference `/boot/loader/loader.conf`.
- `pyxisos.conf.example`: Reference `/boot/loader/entries/pyxisos.conf`.
- `windows.conf.example`: Reference `/boot/loader/entries/windows.conf` for UEFI Windows dual-boot.

## Target Hardware Boot Layout

- Root Partition: `/dev/nvme0n1p7` (UUID: `00955744-524c-403a-99f2-69818444cd30`)
- PyxisOS Boot/ESP: `/dev/nvme0n1p6` (UUID: `76B7-515F`)
- Windows ESP: `/dev/nvme0n1p2` (UUID: `9A05-B7D2`)
- Boot Manager: `systemd-boot`
