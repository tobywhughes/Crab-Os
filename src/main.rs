#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &[u8] = b"Hello, welcome to CRAB OS";

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    welcome_text();
    panic!("End of test code reached. Entering panic.");
    loop {}
}


pub fn welcome_text() {
    use core::fmt::Write;

    println!("Hello, welcome to CRAB OS");

    let format_example: u8 = 100;
    println!("All humans and crabs are {}% welcome", format_example);
}