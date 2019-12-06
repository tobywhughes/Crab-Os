#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;
mod serial;

static HELLO: &[u8] = b"Hello, welcome to CRAB OS";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


#[panic_handler]
#[cfg(test)]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("Error: {}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    welcome_text();

    #[cfg(test)]
    test_main();
    panic!("End of test code reached. Entering panic.");
    loop {}
}


pub fn welcome_text() {
    use core::fmt::Write;

    println!("Hello, welcome to CRAB OS");

    let format_example: u8 = 100;
    println!("All humans and crabs are {}% welcome", format_example);
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests for crab_os", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success)
}

#[test_case]
fn test_trivial_assertion() {
    serial_print!("Making sure 1 still equals 1 (and that our test runner is working)... ");
    assert_eq!(1, 1);
    serial_println!("[ok]")
}