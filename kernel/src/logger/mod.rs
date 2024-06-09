// Remixed from https://github.com/rust-osdev/bootloader/blob/main/common/src/lib.rs licensed under the MIT license

use core::fmt::Write;

use bootloader_api::info::FrameBufferInfo;
use conquer_once::spin::OnceCell;
use spinning_top::Spinlock;

use crate::framebuffer::FrameBufferWriter;

static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();

pub fn init_logger(fb: &'static mut [u8], info: FrameBufferInfo) {
    let logger = LOGGER.get_or_init(move || {
        LockedLogger::new(fb, info)
    });
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
        if let Some(fb) = &self.framebuffer {
            let mut fb = fb.lock();
            writeln!(fb, "{:5}: {}", record.level(), record.args()).unwrap();
        }
    }

    fn flush(&self) {}
}
