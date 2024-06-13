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

extern crate alloc;

use bootloader_api::{entry_point, BootInfo, BootloaderConfig};
use bs_kernel::{allocator, gdt, interrupts, logger::init_logger, memory::{self, BootInfoFrameAllocator}, println, serial_println};
use x86_64::VirtAddr;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(bootloader_api::config::Mapping::Dynamic);
    config
};

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

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset.take().unwrap());

    let mut mapper = unsafe { memory::init(physical_memory_offset) };

    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_regions)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

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

entry_point!(kernel_init, config = &BOOTLOADER_CONFIG);
