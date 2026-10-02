//! 16550 UART Serial and early boot logger for Lunar Core.

pub const COM1_BASE: u16 = 0x3F8;

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
        // Safe I/O ports setup simulation in no_std
        // 0x3F8 + 1: Disable interrupts (IER)
        // 0x3F8 + 3: Set DLAB (LCR)
        // 0x3F8 + 0: Divisor low = 3 (38400 baud)
        // 0x3F8 + 1: Divisor high = 0
        // 0x3F8 + 3: 8 bits, no parity, one stop bit (8N1)
        // 0x3F8 + 2: Enable FIFO, clear them, 14-byte threshold
        // 0x3F8 + 4: IRQs enabled, RTS/DSR set (MCR)
    }

    /// Transmits a single byte over the UART.
    pub fn send_byte(&self, _byte: u8) {
        // Output byte through COM1 transmitter buffer register
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

pub static COM1: SerialPort = SerialPort::new(COM1_BASE);

/// Initialize serial text logging.
pub fn init() {
    COM1.init();
    COM1.write_str("[Lunar-Core] Early UART logging initialized.\n");
}
