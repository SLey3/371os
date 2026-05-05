#![no_std]
#![no_main]

mod colors;
mod vga;
mod img;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    colors::image();
    loop {}
}