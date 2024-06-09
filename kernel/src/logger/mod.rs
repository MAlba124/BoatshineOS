// Remixed from https://github.com/rust-osdev/bootloader/blob/main/common/src/lib.rs licensed under the MIT license

use core::fmt::{self, Write};

use bootloader_api::info::FrameBufferInfo;
use conquer_once::spin::OnceCell;
use spinning_top::Spinlock;
use x86_64::instructions::port::{PortGeneric, ReadWriteAccess};

use crate::framebuffer::FrameBufferWriter;

static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();
lazy_static::lazy_static! {
    static ref DEBUG_SERIAL_PORT: Spinlock<DebugSerialPort> = Spinlock::new(DebugSerialPort::default());
}

pub fn init_logger(fb: &'static mut [u8], info: FrameBufferInfo) {
    let logger = LOGGER.get_or_init(move || LockedLogger::new(fb, info));
    log::set_logger(logger).expect("logger already set");
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Logger initialized");
}

struct LockedLogger {
    framebuffer: Option<Spinlock<FrameBufferWriter>>,
}

impl LockedLogger {
    pub fn new(fb: &'static mut [u8], info: FrameBufferInfo) -> Self {
        Self {
            framebuffer: Some(Spinlock::new(FrameBufferWriter::new(fb, info))),
        }
    }
}

impl log::Log for LockedLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        use x86_64::instructions::interrupts;

        interrupts::without_interrupts(|| {
            if let Some(fb) = &self.framebuffer {
                let mut fb = fb.lock();
                writeln!(fb, "{:5}: {}", record.level(), record.args()).unwrap();
            }
        });
    }

    fn flush(&self) {}
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        match LOGGER.get() {
            Some(logger) => {
                if let Some(fb) = &logger.framebuffer {
                    let mut fb = fb.lock();
                    write!(fb, "{}", args).unwrap();
                }
            }
            None => {}
        }
    });
}

struct DebugSerialPort {
    port: PortGeneric<u8, ReadWriteAccess>,
}

impl Default for DebugSerialPort {
    fn default() -> Self {
        Self {
            port: PortGeneric::new(0xE9),
        }
    }
}

impl DebugSerialPort {
    fn write_byte(&mut self, b: u8) {
        unsafe {
            self.port.write(b);
        }
    }
}

impl fmt::Write for DebugSerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.write_byte(b);
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn _serial_print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        DEBUG_SERIAL_PORT
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::logger::_serial_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}
