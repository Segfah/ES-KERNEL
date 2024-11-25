// src/main.rs

#![no_std] // ne pas lier la bibliothèque standard Rust
#![no_main] // désactiver tous les points d'entrée Rust

mod arch    { pub mod boot; }
mod vga     { pub mod vga_buffer; }
mod screen;
mod keyboard;
pub mod gdt;

use crate::vga::vga_buffer::{ColorCode, Color};
use crate::vga::vga_buffer;
use crate::screen::Screen;
use core::panic::PanicInfo;
use core::arch::asm;
use crate::keyboard::{keyboard_to_ascii};

use crate::gdt::load_gdt;
use crate::gdt::test_gdt;


/// Cette fonction est invoquée lorsque le système panique
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Funcion para iteractuar con los puertos
pub fn inb(port: u16) -> u8 {
	let mut input_byte: u8;
	unsafe { asm!("in al, dx", in("dx") port, out("al") input_byte); }
	input_byte
}

/// Punto de entrada del bootloader
#[no_mangle]
pub extern "C" fn _start() {
    load_gdt();
    print_stack();
    //test_gdt();
    let mut screens = [
        Screen::new(ColorCode::new(Color::Yellow, Color::Black)),
        Screen::new(ColorCode::new(Color::Cyan, Color::Black)),
    ];
    let mut current_screen = 0;
    //screens[current_screen].clear();
    println!("{}", "42");
	loop {
		if inb(0x64) & 1 != 0 {
			let inb = inb(0x60);
			let keycode = keyboard_to_ascii(inb);

            match keycode {
                '1' | '2' => {
                    let new_screen = (keycode as u8 - b'1') as usize;
                    if new_screen != current_screen {
                        screens[current_screen].clear();
                        current_screen = new_screen;
                        println!("Switched to screen {}\n", keycode);
                    }
                }
                '\t' => {
                    screens[current_screen].write_str("    ");
                }
                _ => {
                    if keycode != '\0' {
                        screens[current_screen].write_char(keycode);
                    }
                }
            }
	    }
    }
}


// Imprimir la pila del kernel para depuración
pub fn print_stack() {
    unsafe {
        let esp: u32;
        asm!("mov {}, esp", out(reg) esp);
        println!("Valor del stack pointer: {:#010x}", esp);
        // Imprimir algunos valores desde el stack para depuración
        let stack_ptr = esp as *const u32;
        for i in 0..10 {  // Imprimir los primeros 10 elementos del stack
            let value = *stack_ptr.offset(i);
            println!("Stack[{}]: {:#010x}", i, value);
        }
    }
}