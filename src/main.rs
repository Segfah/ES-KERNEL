// src/main.rs

#![no_std] // ne pas lier la bibliothèque standard Rust
#![no_main] // désactiver tous les points d'entrée Rust

use core::panic::PanicInfo;

/// Cette fonction est invoquée lorsque le système panique
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Punto de entrada del bootloader
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // cette fonction est le point d'entrée, puisque le lieur cherche une fonction
    // nommée `_start` par défaut
    let vga_buffer = 0xb8000 as *mut u8;
    let message = b"42";

    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x0F; // Color blanco sobre negro
        }
    }

    loop {}
}
