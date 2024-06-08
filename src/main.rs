#![no_std]
#![no_main]

use core::panic::PanicInfo;
use boatshineos::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();
    panic!("main() exited");
    loop {}
}

fn main() {
    println!("Kernel running");
}
