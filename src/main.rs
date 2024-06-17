// src/main.rs

#![no_std] // ne pas lier la bibliothèque standard Rust
#![no_main] // désactiver tous les points d'entrée Rust


mod arch    { pub mod boot; }
mod vga     { pub mod vga_buffer; }
use crate::vga::vga_buffer;

use core::panic::PanicInfo;



/// Cette fonction est invoquée lorsque le système panique
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Punto de entrada del bootloader
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}