#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let ints: [i32; 3] = [1819043144, 1870078063, 560229490];

    unsafe {
        // Cast ints to a byte pointer
        let msg_ptr = &ints as *const i32 as *const u8;

        // VGA text buffer address
        let vga_buffer = 0xb8000 as *mut u8;

        // Write 12 characters ("Hello World!")
        let mut i = 0;
        while i < 12 {
            // Write ASCII byte
            *vga_buffer.add(i * 2) = *msg_ptr.add(i);

            // Write color byte (white on black)
            *vga_buffer.add(i * 2 + 1) = 0x0F;

            i += 1;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}