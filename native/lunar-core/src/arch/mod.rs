//! CPU Architecture abstraction and x86_64 hardware descriptor tables.

/// CPU Architecture abstraction trait.
pub trait CpuArch {
    /// Initialize CPU-specific features (GDT, IDT, Paging)
    fn init();
}

/// x86_64 segment selector flags and GDT entry descriptor.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct GdtDescriptor {
    pub limit_low: u16,
    pub base_low: u16,
    pub base_middle: u8,
    pub access: u8,
    pub granularity: u8,
    pub base_high: u8,
}

impl GdtDescriptor {
    pub const fn null() -> Self {
        Self {
            limit_low: 0,
            base_low: 0,
            base_middle: 0,
            access: 0,
            granularity: 0,
            base_high: 0,
        }
    }

    pub const fn kernel_code64() -> Self {
        Self {
            limit_low: 0xFFFF,
            base_low: 0,
            base_middle: 0,
            access: 0x9A,      // Present, Ring 0, Code, Executable, Readable
            granularity: 0xAF, // 64-bit long mode, 4KB granularity
            base_high: 0,
        }
    }

    pub const fn kernel_data64() -> Self {
        Self {
            limit_low: 0xFFFF,
            base_low: 0,
            base_middle: 0,
            access: 0x92,      // Present, Ring 0, Data, Writable
            granularity: 0xCF, // 4KB granularity
            base_high: 0,
        }
    }
}

/// Pointer to GDT for the `lgdt` instruction.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct GdtPointer {
    pub limit: u16,
    pub base: u64,
}

/// x86_64 Interrupt Descriptor Table (IDT) gate descriptor.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct IdtEntry {
    pub offset_low: u16,
    pub selector: u16,
    pub ist: u8,
    pub type_attr: u8,
    pub offset_mid: u16,
    pub offset_high: u32,
    pub zero: u32,
}

impl IdtEntry {
    pub const fn missing() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            ist: 0,
            type_attr: 0,
            offset_mid: 0,
            offset_high: 0,
            zero: 0,
        }
    }

    pub fn new(handler: u64, selector: u16, ist: u8, type_attr: u8) -> Self {
        Self {
            offset_low: handler as u16,
            selector,
            ist,
            type_attr,
            offset_mid: (handler >> 16) as u16,
            offset_high: (handler >> 32) as u32,
            zero: 0,
        }
    }
}

/// Pointer to IDT for the `lidt` instruction.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct IdtPointer {
    pub limit: u16,
    pub base: u64,
}

/// Control Register flags.
pub struct ControlRegisters;

impl ControlRegisters {
    pub const CR0_PROTECTED_MODE: u64 = 1 << 0;
    pub const CR0_PAGING: u64 = 1 << 31;
    pub const CR4_PAE: u64 = 1 << 5;
    pub const CR4_PAGE_GLOBAL: u64 = 1 << 7;
    pub const CR4_OSFXSR: u64 = 1 << 9;
    pub const CR4_OSXMMEXCPT: u64 = 1 << 10;
    pub const EFER_LONG_MODE_ENABLE: u64 = 1 << 8;
    pub const EFER_NO_EXECUTE_ENABLE: u64 = 1 << 11;
}

#[inline]
pub unsafe fn hlt() {
    core::arch::asm!("hlt", options(nomem, nostack));
}

#[inline]
pub unsafe fn cli() {
    core::arch::asm!("cli", options(nomem, nostack));
}

#[inline]
pub unsafe fn sti() {
    core::arch::asm!("sti", options(nomem, nostack));
}

#[inline]
pub unsafe fn load_gdt(ptr: &GdtPointer) {
    core::arch::asm!("lgdt [{}]", in(reg) ptr, options(readonly, nostack, preserves_flags));
}

#[inline]
pub unsafe fn load_idt(ptr: &IdtPointer) {
    core::arch::asm!("lidt [{}]", in(reg) ptr, options(readonly, nostack, preserves_flags));
}

#[inline]
pub unsafe fn read_cr0() -> u64 {
    let cr0: u64;
    core::arch::asm!("mov {}, cr0", out(reg) cr0, options(nomem, nostack));
    cr0
}

#[inline]
pub unsafe fn read_cr2() -> u64 {
    let cr2: u64;
    core::arch::asm!("mov {}, cr2", out(reg) cr2, options(nomem, nostack));
    cr2
}

#[inline]
pub unsafe fn read_cr3() -> u64 {
    let cr3: u64;
    core::arch::asm!("mov {}, cr3", out(reg) cr3, options(nomem, nostack));
    cr3
}

#[inline]
pub unsafe fn write_cr3(val: u64) {
    core::arch::asm!("mov cr3, {}", in(reg) val, options(nomem, nostack));
}

static EARLY_GDT: [GdtDescriptor; 3] = [
    GdtDescriptor::null(),
    GdtDescriptor::kernel_code64(),
    GdtDescriptor::kernel_data64(),
];

/// X86_64 architecture implementation for Lunar Core.
pub struct X86_64;

impl CpuArch for X86_64 {
    fn init() {
        unsafe {
            let gdt_ptr = GdtPointer {
                limit: (core::mem::size_of_val(&EARLY_GDT) - 1) as u16,
                base: EARLY_GDT.as_ptr() as u64,
            };
            load_gdt(&gdt_ptr);
        }
    }
}

