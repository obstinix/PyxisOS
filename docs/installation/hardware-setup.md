# ThinkPad X1 Carbon 5th Gen Hardware Reference

This document outlines the hardware reference specifications, module configurations, and firmware prerequisites for running PyxisOS.

---

## Hardware Profile

- **Device:** Lenovo ThinkPad X1 Carbon (5th Generation, Type 20HR / 20HQ)
- **CPU:** Intel Core i7-7600U @ 2.80GHz (Kaby Lake, 2 cores / 4 threads)
- **Memory:** 16 GB LPDDR3 1866 MHz (soldered)
- **GPU:** Intel HD Graphics 620
- **Storage:** NVMe M.2 2280 SSD
- **Network:** Intel Dual Band Wireless-AC 8265 (802.11ac + Bluetooth 4.2)
- **Audio:** Realtek ALC3268 High Definition Audio
- **Display:** 14.0" IPS FHD (1920x1080) anti-glare
- **Input:** UltraNav (TrackPoint + ClickPad)

---

## UEFI / BIOS Settings

To ensure compatibility with UEFI dual boot and early NVMe root discovery:

1. **Security:**
   - **Secure Boot:** Disabled (required for custom kernels and unsigned EFI loaders).
   - **Intel Platform Trust Technology (PTT):** Enabled.
2. **Startup:**
   - **UEFI/Legacy Boot:** UEFI Only.
   - **CSM Support:** No.
3. **Config -> Power:**
   - **Sleep State:** Linux (S3 support if available in BIOS).
4. **Config -> Thunderbolt 3:**
   - **Security Level:** User Authorization or No Security (for Linux TB3 docking).

---

## Kernel Module Requirements

The following modules must be included in early userspace (`/etc/mkinitcpio.conf` `MODULES=(...)`):

```text
nvme vfat ext4 intel_agp i915
```

Running `mkinitcpio -P` compiles these drivers into `/boot/initramfs-linux.img`, ensuring the NVMe storage controller is discovered before root mounting.
