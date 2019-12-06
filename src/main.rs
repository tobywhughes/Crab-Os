#![no_std]
#![no_main]


#![feature(custom_test_frameworks)]
#![test_runner(crab_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use crab_os::println;
use crab_os::asm;

static HELLO: &[u8] = b"Hello, welcome to CRAB OS";



#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


#[panic_handler]
#[cfg(test)]
fn panic(info: &PanicInfo) -> ! {
    crab_os::test_panic_handler(info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    welcome_text();

    #[cfg(test)]
    test_main();
    panic!("End of test code reached. Entering panic.");
    loop {}
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn init() {
    asm::nop();
}


pub fn welcome_text() {
    use core::fmt::Write;

    println!("Hello, welcome to CRAB OS");

    let format_example: u8 = 100;
    println!("All humans and crabs are {}% welcome", format_example);
}
