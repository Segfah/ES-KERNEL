// in src/vga_buffer.rs

/*
** ---------------------------
** Colores
** ---------------------------
*/

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/*
** ---------------------------
** Caracteres de pantalla
** ---------------------------
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/*
** ---------------------------
** función de escritura
** ---------------------------
*/

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        let row = self.row_position;
        let col = self.column_position;
        let color_code = self.color_code;

        match byte {
            b'\n' => self.new_line(),
            0x08 => {
                if self.column_position > 0 {
                    self.column_position -= 1;
                    let col = self.column_position;
                    self.buffer.chars[row][col] = ScreenChar {
                        ascii_character: b' ',
                        color_code,
                    };
                }
                update_cursor(row, col);
            },
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let col = self.column_position;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
                update_cursor(row, col + 1);
            }
        }
    }

    fn new_line(&mut self) {
        // if self.row_position + 1 >= BUFFER_HEIGHT {
        //     self.scroll_up();
        // } else {
        //     self.row_position += 1;
        // }
        // self.clear_row(BUFFER_HEIGHT - 1);
        // self.column_position = 0;
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = character;
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
        update_cursor(BUFFER_HEIGHT - 1, 0);
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col]= blank;
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' | 0x08 => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }
    
}

// Implementación de la función write! de la macro de formato de Rust (para poder imprimir numero flotantes y otras cosas.)
use core::fmt;
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}


use lazy_static::lazy_static;
use spin::Mutex;
use core::arch::asm;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        row_position: BUFFER_HEIGHT - 1,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

// Funcion para iteractuar con los puertos
pub fn outb(port: u16, cmd: u8) {
    unsafe { asm!("out dx, al", in("dx") port, in("al") cmd); }
}

pub fn inb(port: u16) -> u8 {
    let mut input_byte: u8;
    unsafe { asm!("in al, dx", in("dx") port, out("al") input_byte); }
    input_byte
}

pub fn update_cursor(row: usize, col: usize) {

    let pos = row * BUFFER_WIDTH + col;

    unsafe {
        outb(0x3D4, 0x0F);
        outb(0x3D5, (pos & 0xFF) as u8);
        
        outb(0x3D4, 0x0E);
        outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
    }
}