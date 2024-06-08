#![no_std]
#![no_main]

use core::panic::PanicInfo;

use boatshineos::vga;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::write_hello_world();
    loop {}
}
