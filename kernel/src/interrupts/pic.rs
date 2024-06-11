use pic8259::ChainedPics;

use super::InterruptIndex;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub fn init() {
    unsafe {
        PICS.lock().initialize();
    }
}

#[inline(always)]
pub fn notify_end(index: InterruptIndex) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(index.as_u8());
    }
}
