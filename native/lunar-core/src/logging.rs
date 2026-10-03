//! 16550 UART Serial and early boot logger for Lunar Core.

use core::fmt;

pub const COM1_BASE: u16 = 0x3F8;

#[inline]
pub unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") val,
        options(nomem, nostack, preserves_flags)
    );
}

#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!(
        "in al, dx",
        out("al") value,
        in("dx") port,
        options(nomem, nostack, preserves_flags)
    );
    value
}

/// 16550 UART Serial Port abstraction.
pub struct SerialPort {
    base_port: u16,
}

impl SerialPort {
    pub const fn new(base_port: u16) -> Self {
        Self { base_port }
    }

    pub const fn port(&self) -> u16 {
        self.base_port
    }

    /// Initializes COM1 serial port (baud 38400, 8N1, FIFO enabled).
    pub fn init(&self) {
        unsafe {
            // Disable all interrupts
            outb(self.base_port + 1, 0x00);
            // Enable DLAB (set baud rate divisor)
            outb(self.base_port + 3, 0x80);
            // Set divisor to 3 (lo byte) 38400 baud
            outb(self.base_port + 0, 0x03);
            // (hi byte)
            outb(self.base_port + 1, 0x00);
            // 8 bits, no parity, one stop bit
            outb(self.base_port + 3, 0x03);
            // Enable FIFO, clear them, with 14-byte threshold
            outb(self.base_port + 2, 0xC7);
            // IRQs enabled, RTS/DSR set
            outb(self.base_port + 4, 0x0B);
        }
    }

    /// Checks if the transmitter holding register is empty.
    pub fn is_transmit_empty(&self) -> bool {
        unsafe { (inb(self.base_port + 5) & 0x20) != 0 }
    }

    /// Transmits a single byte over the UART.
    pub fn send_byte(&self, byte: u8) {
        while !self.is_transmit_empty() {
            core::hint::spin_loop();
        }
        unsafe {
            outb(self.base_port, byte);
        }
    }

    /// Writes a string slice to serial console.
    pub fn write_str(&self, s: &str) {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.send_byte(b'\r');
            }
            self.send_byte(byte);
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        SerialPort::write_str(self, s);
        Ok(())
    }
}

pub static COM1: SerialPort = SerialPort::new(COM1_BASE);

/// Initialize serial text logging.
pub fn init() {
    COM1.init();
    COM1.write_str("[Lunar-Core] Early UART logging initialized.\n");
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    struct SerialWriter;
    impl Write for SerialWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            COM1.write_str(s);
            Ok(())
        }
    }
    let mut writer = SerialWriter;
    let _ = writer.write_fmt(args);
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::logging::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}
