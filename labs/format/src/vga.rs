// src/vga.rs

pub struct Writer;

pub fn str_to_vga(s: &str) {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in s.as_bytes().iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // Cyan color
        }
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        str_to_vga(s); 
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    let mut w = Writer;
    w.write_fmt(args).unwrap();
}
