#![no_std]
#![no_main]



use core::panic::PanicInfo;
use crab_os::{QemuExitCode, exit_qemu, serial_println, serial_print};



#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit_qemu(QemuExitCode::Success);
    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();

    loop {}
}



fn should_fail() {
    assert_eq!(0, 1);
    exit_qemu(QemuExitCode::Failed);

}