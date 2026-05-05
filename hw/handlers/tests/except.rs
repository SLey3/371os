#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use osirs::{exit_qemu, QemuExitCode, serial_println};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    serial_println!("test_breakpoint...");
    osirs::init();
    
    x86_64::instructions::interrupts::int3();
    
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    osirs::test_panic_handler(info)
}