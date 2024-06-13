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

#![feature(abi_x86_interrupt)]
#![no_std]

extern crate alloc;

pub mod framebuffer;
pub mod gdt;
pub mod interrupts;
pub mod logger;
pub mod macros;
pub mod memory;
pub mod panic;
pub mod allocator;
