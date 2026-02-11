#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
#[allow(unconditional_recursion)]
fn panic(_info: &PanicInfo) -> ! {
    panic(_info)
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}
