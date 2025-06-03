use crate::port::Port;
use core::{fmt, ptr};
use lazy_static::lazy_static;
use spin::Mutex;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
    pub static ref CURSOR: Mutex<Cursor> = Mutex::new(Cursor::new());
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

#[macro_export]
macro_rules! clear_screen {
    () => {
        $crate::vga_buffer::WRITER.lock().clear_screen()
    };
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

pub struct Cursor {
    command_port: Port<u8>,
    data_port: Port<u8>,
}

impl Cursor {
    fn new() -> Cursor {
        let mut cursor = Cursor {
            command_port: Port::new(0x3D4),
            data_port: Port::new(0x3D5),
        };
        cursor.enable_cursor(14, 15);
        cursor
    }

    fn move_cursor(&mut self, pos: u16) {
        const HIGH_BYTE: u8 = 0x0E;
        const LOW_BYTE: u8 = 0x0F;

        unsafe {
            self.command_port.write(HIGH_BYTE);
            self.data_port.write(((pos >> 8) & 0xFF) as u8);
            self.command_port.write(LOW_BYTE);
            self.data_port.write((pos & 0xFF) as u8);
        };
    }

    fn enable_cursor(&mut self, cursor_start: u8, cursor_end: u8) {
        const START_REGISTER: u8 = 0x0A;
        const END_REGISTER: u8 = 0x0B;

        unsafe {
            self.command_port.write(START_REGISTER);
            let mut current_data = self.data_port.read();
            self.data_port
                .write((current_data & 0xC0) | (cursor_start & 0x1F));
            self.command_port.write(END_REGISTER);
            current_data = self.data_port.read();
            self.data_port
                .write((current_data & 0xE0) | (cursor_end & 0x1F));
        }
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    fn write_byte_at(&mut self, byte: u8, row: usize, col: usize) {
        let color_code = self.color_code;
        unsafe {
            ptr::write_volatile(
                &mut self.buffer.chars[row][col],
                ScreenChar {
                    ascii_character: byte,
                    color_code,
                },
            );
        };
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b' '..=b'~' => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                self.write_byte_at(byte, row, self.column_position);

                self.column_position += 1;
                CURSOR
                    .lock()
                    .move_cursor((row * BUFFER_WIDTH + self.column_position) as u16);
            }
            _ => self.write_byte(b'*'),
        }
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                unsafe {
                    let character = ptr::read_volatile(&self.buffer.chars[row][col]);
                    ptr::write_volatile(&mut self.buffer.chars[row - 1][col], character);
                };
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
        let row = BUFFER_HEIGHT - 1;
        CURSOR
            .lock()
            .move_cursor((row * BUFFER_WIDTH + self.column_position) as u16);
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.write_byte_at(b' ', row, col);
        }
    }

    pub fn clear_screen(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.column_position = 0;
        CURSOR
            .lock()
            .move_cursor(((BUFFER_HEIGHT - 1) * BUFFER_WIDTH) as u16);
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
