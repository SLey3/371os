use volatile::Volatile;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0, Blue = 1, Green = 2, Cyan = 3, Red = 4, Magenta = 5, Brown = 6,
    LightGray = 7, DarkGray = 8, LightBlue = 9, LightGreen = 10, LightCyan = 11,
    LightRed = 12, Pink = 13, Yellow = 14, White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

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
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    pub column_position: usize,
    pub color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        // Simple implementation for snake
        let row = BUFFER_HEIGHT - 1;
        let col = self.column_position;
        self.buffer.chars[row][col].write(ScreenChar {
            ascii_character: byte,
            color_code: self.color_code,
        });
        self.column_position += 1;
    }
}

pub fn write_pixel(x: usize, y: usize, byte: u8, color: Color) {
    let mut writer = WRITER.lock();
    let color_code = ColorCode::new(color, Color::Black);
    if x < BUFFER_WIDTH && y < BUFFER_HEIGHT {
        writer.buffer.chars[y][x].write(ScreenChar {
            ascii_character: byte,
            color_code,
        });
    }
}

pub fn clear_screen() {
    let mut writer = WRITER.lock();
    for y in 0..BUFFER_HEIGHT {
        for x in 0..BUFFER_WIDTH {
            writer.buffer.chars[y][x].write(ScreenChar {
                ascii_character: b' ',
                color_code: ColorCode::new(Color::Black, Color::Black),
            });
        }
    }
    writer.column_position = 0;
}