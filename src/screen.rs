use crate::vga::vga_buffer::{ColorCode, Color, BUFFER_HEIGHT, Writer, Buffer};
use crate::vga::vga_buffer;

/// Structure representing a single screen
pub struct Screen {
    writer: Writer,
}

impl Screen {
    /// Create a new screen with a specific color scheme
    pub fn new(color_code: ColorCode) -> Screen {
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
    pub fn clear(&mut self) {
        for _ in 0..BUFFER_HEIGHT {
            self.writer.new_line();
        }
        self.writer.column_position = 0;
        self.writer.row_position = BUFFER_HEIGHT - 1;
    }

    /// Write a character to the screen
    pub fn write_char(&mut self, c: char) {
        self.writer.write_byte(c as u8);
    }

}