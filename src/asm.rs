
pub fn nop() {
    unsafe {
        asm!("NOP");
    }
}

pub fn enable_interrupts() {
    unsafe {
        asm!("STI");
    }
}

pub fn disable_interrupts() {
    unsafe {
        asm!("CLI");
    }
}

use crate::gdt::Descriptor;

pub fn load_global_descriptor_table(table_pointer: &Descriptor) {
    unsafe {
        asm!("LGDT ($0)" : : "r"(table_pointer));
    }
}


pub fn get_global_descriptor_table() ->Descriptor {
    let mut descriptor: Descriptor = Descriptor {
        base: 0,
        limit: 0
    };
    unsafe {
        asm!("SGDT $0" : "=*m"(&mut descriptor));
    }
    return descriptor
}

