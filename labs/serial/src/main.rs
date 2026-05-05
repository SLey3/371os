#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // If we are in a test environment, run the tests
    #[cfg(test)]
    test_main();

    loop {}
}

/// Our custom test runner that uses the serial port for output
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    // After tests finish, shut down QEMU
    exit_qemu(QemuExitCode::Success);
}

/// Standard panic handler for the kernel
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // In a real kernel, you might print to VGA here
    loop {}
}

/// Panic handler during tests: redirects output to the serial port
#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// --- QEMU Exit Logic ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10, // Result of (0x10 << 1) | 1 = 33
    Failed = 0x11,  // Result of (0x11 << 1) | 1 = 35
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        // The port 0xf4 is the iobase of the isa-debug-exit device
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}