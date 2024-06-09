// Copyright (C) 2024  MAlba124 <marlhan@proton.me>
//
// BoatshineOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// BoatshineOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with BoatshineOS.  If not, see <https://www.gnu.org/licenses/>.
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bs_kernel::logger::init_logger;
use bootloader_api::{entry_point, BootInfo};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{}", info);
    loop {}
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // println!("Kernel running");
    match boot_info.framebuffer.as_mut() {
        Some(&mut ref mut framebuffer) => {
            let info = framebuffer.info();
            init_logger(framebuffer.buffer_mut(), info);
        }
        None => panic!("no framebuffer"),
    }
    // let mut fb = &'static mut .unwrap();
    // let fb_info = fb.info();
    // init_logger(fb.buffer_mut(), fb_info);

    loop {}
}

entry_point!(kernel_main);
