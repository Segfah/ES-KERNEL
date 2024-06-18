// src/main.rs

#![no_std] // ne pas lier la bibliothèque standard Rust
#![no_main] // désactiver tous les points d'entrée Rust


mod arch    { pub mod boot; }
mod vga     { pub mod vga_buffer; }
use crate::vga::vga_buffer;

use core::panic::PanicInfo;



/// Cette fonction est invoquée lorsque le système panique
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


use core::arch::asm;


pub fn outb(port: u16, cmd: u8) {
	unsafe { asm!("out dx, al", in("dx") port, in("al") cmd); }
}

pub fn inb(port: u16) -> u8 {
	let mut input_byte: u8;
	unsafe { asm!("in al, dx", in("dx") port, out("al") input_byte); }
	input_byte
}

/// Punto de entrada del bootloader
#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");
	loop {
		if inb(0x64) & 1 != 0 {
			let inb = inb(0x60);
            println!("intento{}intento\n", inb);
			let keycode = keyboard_to_ascii(inb);
            println!("nani{}nani\n", keycode);

			if keycode != '\0'
				{print!("{}", keycode);}
		}
	}
}

static KEYBOARD: [char; 256] = [
    '\0',    // 0x00
    '\0',    // 0x01 - Escape
    '1',     // 0x02 - 1
    '2',     // 0x03 - 2
    '3',     // 0x04 - 3
    '4',     // 0x05 - 4
    '5',     // 0x06 - 5
    '6',     // 0x07 - 6
    '7',     // 0x08 - 7
    '8',     // 0x09 - 8
    '9',     // 0x0A - 9
    '0',     // 0x0B - 0
    '-',     // 0x0C - -
    '=',     // 0x0D - =
    '\x08',  // 0x0E - Retroceso (Backspace)
    '\t',    // 0x0F - Tab
    'q',     // 0x10 - q
    'w',     // 0x11 - w
    'e',     // 0x12 - e
    'r',     // 0x13 - r
    't',     // 0x14 - t
    'y',     // 0x15 - y
    'u',     // 0x16 - u
    'i',     // 0x17 - i
    'o',     // 0x18 - o
    'p',     // 0x19 - p
    '[',     // 0x1A - [
    ']',     // 0x1B - ]
    '\n',    // 0x1C - Enter
    '\0',    // 0x1D - Control (left)
    'a',     // 0x1E - a
    's',     // 0x1F - s
    'd',     // 0x20 - d
    'f',     // 0x21 - f
    'g',     // 0x22 - g
    'h',     // 0x23 - h
    'j',     // 0x24 - j
    'k',     // 0x25 - k
    'l',     // 0x26 - l
    ';',     // 0x27 - ;
    '\'',    // 0x28 - '
    '`',     // 0x29 - `
    '\0',    // 0x2A - Shift (left)
    '\\',    // 0x2B - \
    'z',     // 0x2C - z
    'x',     // 0x2D - x
    'c',     // 0x2E - c
    'v',     // 0x2F - v
    'b',     // 0x30 - b
    'n',     // 0x31 - n
    'm',     // 0x32 - m
    ',',     // 0x33 - ,
    '.',     // 0x34 - .
    '/',     // 0x35 - /
'\0', '\0', '\0', '\0', '\0', '\0', '\0',
'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'];

fn keyboard_to_ascii(key: u8) -> char {
	return KEYBOARD[key as usize];
}