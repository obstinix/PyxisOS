# Temporary Milestones (Track A Substrate)

This document clarifies the distinction between the **Current / Temporary Implementation (Track A)** and the **Long-Term Architectural Vision (Track B)** of PyxisOS.

---

## Architectural Context

PyxisOS uses a deliberate two-track strategy to avoid stalling high-level UX and AI research while deep kernel engineering takes place:

```
Track A (Current Functional Implementation)
  Linux Operational Foundation -> Linux Kernel Substrate -> systemd-boot -> KDE Plasma/SDDM -> Pyxis CLI
  (Enables daily usability, hardware validation, and spatial UI experimentation)

Track B (Long-Term Native Vision)
  Freestanding Bootstrap -> Lunar Core (Rust Microkernel) -> Aegis Isolation -> Native Spatial Compositor
  (From-scratch deep-systems engineering)
```

---

## Temporary Implementation Achievements

The following components are operational and verified, but represent the **intermediate substrate**, not the final native state of PyxisOS:

1. **PyxisOS 0.1.0 "Lunar-Pyxis" (Beta):**
   - Built on a Linux-based operational foundation to provide a modern, reliable userspace.
   - Provides full driver support for ThinkPad hardware (Wi-Fi, trackpoint, power management).

2. **KDE Plasma 6 + SDDM:**
   - Serves as the temporary graphical shell while the custom spatial desktop environment is researched and prototyped.
   - Allows windowed multitasking, display scaling, and application management on physical hardware.

3. **systemd-boot 261.2:**
   - Chosen as a clean, minimal UEFI boot manager for dual-booting with Windows 10.
   - Will eventually be complemented or replaced by a custom boot mechanism in Track B.

4. **Pyxis CLI (`/usr/local/bin/pyxis`):**
   - Written in POSIX/Bash as an operational management tool for inspecting the Track A environment.
   - Will transition into a native Rust binary (`native/lunar-cli` or similar) in future native phases.

5. **Pyxis Banner Prototype (`/usr/local/bin/pyxis-banner`):**
   - Terminal branding proof-of-concept displaying the `figlet` slant logo.
   - TTY/getty integration was intentionally paused to keep console login standard during development.

---

## What PyxisOS Is Not (Yet)

- PyxisOS is not yet an independent Linux distribution built from independent source packages.
- PyxisOS does not yet boot on a custom microkernel on physical hardware.
- Nebula is currently an exploratory spatial research initiative (`research/nebula/`), not yet running natively as a standalone Wayland compositor.
