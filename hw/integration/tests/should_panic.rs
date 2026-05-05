#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(integration::_test_runner)]

use core::panic::PanicInfo;
use integration::{QEMU_PASS, QEMU_FAIL, qemu_quit, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("should_panic::test_should_panic... ");
    should_fail();
    serial_println!("[test did not panic]");
    qemu_quit(QEMU_FAIL);
    loop {}
}

fn should_fail() {
    assert!(false);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[Pass]");
    qemu_quit(QEMU_PASS);
    loop {}
}