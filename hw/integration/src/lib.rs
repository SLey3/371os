#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::_test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga;
pub mod serial;

use core::panic::PanicInfo;

// QEMU exit constants
pub const QEMU_PASS: u32 = 0x10;
pub const QEMU_FAIL: u32 = 0x11;

/// Quits QEMU through the isa-debug-exit device
pub fn qemu_quit(exit_code: u32) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code);
    }
}

/// The actual test runner logic
pub fn _test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
        serial_println!("[ok]");
    }
    qemu_quit(QEMU_PASS);
}

/// A panic handler specifically for testing
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    qemu_quit(QEMU_FAIL);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}