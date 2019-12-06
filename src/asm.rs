
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


