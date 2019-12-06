
pub fn nop() {
    unsafe {
        asm!("NOP");
    }
}