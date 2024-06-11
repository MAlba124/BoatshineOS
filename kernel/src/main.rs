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

use bootloader_api::{entry_point, BootInfo};
use bs_kernel::{gdt, interrupts, logger::init_logger, println, serial_print, serial_println};

fn kernel_init(boot_info: &'static mut BootInfo) -> ! {
    match boot_info.framebuffer.as_mut() {
        Some(&mut ref mut framebuffer) => {
            let info = framebuffer.info();
            init_logger(framebuffer.buffer_mut(), info);
        }
        None => panic!("no framebuffer"),
    }

    serial_println!("E9 port working");

    gdt::init();
    interrupts::init_idt();
    interrupts::pic::init();
    x86_64::instructions::interrupts::enable();

    println!("Kernel initialized");

    kernel_main();

    kernel_finished();
}

fn kernel_main() {
    println!("In kernel_main");
}

fn kernel_finished() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

entry_point!(kernel_init);
