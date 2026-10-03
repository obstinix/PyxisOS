# PyxisOS Shell & Interface Architecture

**Track A (Operational Shell) & Track B (Native Interface Research)**

PyxisOS organizes user and console interaction across distinct operational and research tiers:

## Subsystem Tiers

1. **Operational Desktop Shell (Track A):**
   - KDE Plasma 6 running on Wayland via SDDM on physical hardware.
   - Provides daily multi-window productivity, display scaling, and hardware validation.
   - Documented in [`desktop/kde/README.md`](../../desktop/kde/README.md).

2. **Built-in Kernel Console Shell (`userspace/shell/`):**
   - Freestanding C kernel shell executing over serial COM1 and VGA text buffer.
   - Provides commands: `help`, `info`, `mem`, `uptime`, `cat`, `clear`, `reboot`.

3. **Spatial Interface Research (Nebula):**
   - Long-term research investigating 3D spatial window surfaces and native Wayland compositor integration.
   - Documented in [`research/nebula/README.md`](../../research/nebula/README.md).
