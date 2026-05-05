#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(integration::_test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use integration::{println, serial_print};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    integration::test_panic_handler(info)
}

#[test_case]
fn test_println_scrolling() {
    serial_print!("test_println_scrolling... ");
    for i in 0..30 {
        println!("Line {}", i);
    }
}

#[test_case]
fn test_println_wrapping() {
    serial_print!("test_println_wrapping... ");
    println!("{:081x}", 1); 
}