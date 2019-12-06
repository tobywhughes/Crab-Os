#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crab_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use crab_os::{println, serial_print, serial_println};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crab_os::test_panic_handler(info)
}


#[test_case]
fn test_integration_println_does_not_panic() {
    serial_print!("test_integration_println_does_not_panic... ");
    println!("test_integration_println_does_not_panic output");
    serial_println!("[ok]");
}

use crab_os::asm;

#[test_case]
fn test_asm_nop_does_not_panic() {
    serial_print!("test_asm_nop_does_not_panic... ");
    asm::nop();
    serial_println!("[ok]");
}