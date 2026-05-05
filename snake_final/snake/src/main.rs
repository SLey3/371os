#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;


use x86_64::VirtAddr;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::instructions::hlt;
use crate::memory::BootInfoFrameAllocator;

mod memory;
mod vga_buffer;
mod interrupts;
mod allocator;
mod snake;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    vga_buffer::clear_screen();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
    .expect("heap initialization failed");
    
    interrupts::init_idt();
    
    x86_64::instructions::interrupts::enable();

    snake::start_game();

    loop {
        hlt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use crate::vga_buffer::{write_pixel, Color};
    let msg = b"PANIC!";
    for (i, &byte) in msg.iter().enumerate() {
        write_pixel(37 + i, 11, byte, Color::LightRed);
    }
    loop {
        x86_64::instructions::hlt();
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
