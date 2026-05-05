#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let ints: [i32; 3] = [1819043144, 1870078063, 560229490];

    unsafe {
        let msg_ptr = &ints as *const i32 as *const u8;

        let vga_buffer = 0xb8000 as *mut u8;

        let mut i = 0;
        while i < 12 {
            *vga_buffer.offset(i * 2) = *msg_ptr.offset(i);

            *vga_buffer.offset(i * 2 + 1) = 0x0F;

            i += 1;
        }
    }

    loop {}
}

#[panic_handler]
#[allow(unconditional_recursion)]
fn panic(_info: &PanicInfo) -> ! {
    panic(_info)
}