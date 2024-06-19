// src/main.rs

#![no_std] // ne pas lier la bibliothèque standard Rust
#![no_main] // désactiver tous les points d'entrée Rust

mod arch    { pub mod boot; }
mod vga     { pub mod vga_buffer; }

use crate::vga::vga_buffer::{Writer, ColorCode, Color, BUFFER_HEIGHT, Buffer};
use crate::vga::vga_buffer;
use core::panic::PanicInfo;
use core::arch::asm;

/// Cette fonction est invoquée lorsque le système panique
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

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
    let mut screens = [
        Screen::new(ColorCode::new(Color::Yellow, Color::Black)),
        Screen::new(ColorCode::new(Color::Cyan, Color::Black)),
    ];
    let mut current_screen = 0;
    screens[current_screen].clear();
    // enable_cursor(14, 15);
    println!("Hello World{}", "!");
	loop {
		if inb(0x64) & 1 != 0 {
			let inb = inb(0x60);
            // println!("intento {} intento\n", inb);
			let keycode = keyboard_to_ascii(inb);
            // println!("nani {} nani\n", keycode);

            match keycode {
                '1' | '2' => {
                    let new_screen = (keycode as u8 - b'1') as usize;
                    if new_screen != current_screen {
                        screens[current_screen].clear();
                        // screens[current_screen].hide_cursor();
                        current_screen = new_screen;
                        println!("Switched to screen {}\n", keycode);
                        // screens[current_screen].show_cursor();
                    }
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

/// Structure representing a single screen
struct Screen {
    writer: Writer,
}

impl Screen {
    /// Create a new screen with a specific color scheme
    fn new(color_code: ColorCode) -> Screen {
        Screen {
            writer: Writer {
                column_position: 0,
                row_position: BUFFER_HEIGHT - 1,
                color_code,
                buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
            },
        }
    }

    /// Clear the screen
    fn clear(&mut self) {
        for _ in 0..BUFFER_HEIGHT {
            self.writer.new_line();
        }
        self.writer.column_position = 0;
        self.writer.row_position = BUFFER_HEIGHT - 1;
    }

    /// Write a character to the screen
    fn write_char(&mut self, c: char) {
        self.writer.write_byte(c as u8);
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
    '\0',    // 0x36 - Shift (right)
    '*',     // 0x37 - Numpad *
    '\0',    // 0x38 - Alt (left)
    ' ',     // 0x39 - Space
    '\0',    // 0x3A - Caps Lock
    '\0',    // 0x3B - F1
    '\0',    // 0x3C - F2
    '\0',    // 0x3D - F3
    '\0',    // 0x3E - F4
    '\0',    // 0x3F - F5
    '\0',    // 0x40 - F6
    '\0',    // 0x41 - F7
    '\0',    // 0x42 - F8
    '\0',    // 0x43 - F9
    '\0',    // 0x44 - F10
    '\0',    // 0x45 - Num Lock
    '\0',    // 0x46 - Scroll Lock
    '7',     // 0x47 - Numpad 7
    '8',     // 0x48 - Numpad 8
    '9',     // 0x49 - Numpad 9
    '-',     // 0x4A - Numpad -
    '4',     // 0x4B - Numpad 4
    '5',     // 0x4C - Numpad 5
    '6',     // 0x4D - Numpad 6
    '+',     // 0x4E - Numpad +
    '1',     // 0x4F - Numpad 1
    '2',     // 0x50 - Numpad 2
    '3',     // 0x51 - Numpad 3
    '0',     // 0x52 - Numpad 0
    '.',     // 0x53 - Numpad .
    '\0',    // 0x54 - Reserved
    '\0',    // 0x55 - Reserved
    '\0',    // 0x56 - Reserved
    '\0',    // 0x57 - F11
    '\0',    // 0x58 - F12
    '=',     // 0x59 - Numpad =
    '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
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
    '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'
];


fn keyboard_to_ascii(key: u8) -> char {
	return KEYBOARD[key as usize];
}

pub fn enable_cursor(cursor_start: u8, cursor_end: u8) {

    unsafe {
        outb(0x3D4, 0x0A);
        outb(0x3D5, (inb(0x3D5) & 0xC0) | cursor_start);

        outb(0x3D4, 0x0B);
        outb(0x3D5, (inb(0x3D5) & 0xE0) | cursor_end);
    }
}