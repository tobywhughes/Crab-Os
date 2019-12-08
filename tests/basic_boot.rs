#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crab_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


use crab_os::{println};
use core::panic::PanicInfo;

use crab_os::asm;
use crab_os::models::gdt;

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
    println!("test_integration_println_does_not_panic output");
}



#[test_case]
fn test_asm_nop_does_not_panic() {
    asm::nop();
}

#[test_case]
fn test_asm_cli_does_not_panic() {
    asm::disable_interrupts();
}

#[test_case]
fn test_asm_sti_does_not_panic() {
    asm::enable_interrupts();
}


#[test_case]
fn test_asm_lgdt_does_not_panic() {
    let empty_descriptor = gdt::Descriptor {
        limit: 0,
        base: 0,
    };
    asm::load_global_descriptor_table(&empty_descriptor);
}

#[test_case]
fn test_asm_sgdt_does_not_panic() {
    asm::get_global_descriptor_table();
}


#[test_case]
fn test_asm_lgdt_and_sgdt_stores_in_gdt_register_and_retrieves() {
    let expected_descriptor = gdt::Descriptor {
        limit: 1,
        base: 1,
    };
    
    asm::load_global_descriptor_table(&expected_descriptor);
    let actual_descriptor = asm::get_global_descriptor_table();

    assert_eq!(expected_descriptor.limit, actual_descriptor.limit);
    assert_eq!(expected_descriptor.base, actual_descriptor.base);

}