use crate::asm;

pub fn enter_protected_mode() {
    asm::disable_interrupts();
}