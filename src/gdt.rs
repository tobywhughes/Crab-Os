
use crate::asm;
use volatile::Volatile;
use core::ops::{Deref,DerefMut};
use crate::models::gdt::*;
use crate::println;

static mut GLOBAL_DESCRIPTOR_TABLE: GlobalDescriptorTable = GlobalDescriptorTable {segments: [0_u64; GDT_ELEMENTS as usize]};

pub fn enter_protected_mode() {
    println!("ENTERING PROTECTED MODE...\n");
    asm::disable_interrupts();
    let GDT_DESCRIPTOR: Descriptor = create_gdt_definition();
    println!("GDT DESCRIPTOR PRE:  {:?}", GDT_DESCRIPTOR);
    asm::load_global_descriptor_table(&GDT_DESCRIPTOR);
    let temp = asm::get_global_descriptor_table();
    println!("GDT DESCRIPTOR POST: {:?}\n\n", temp);

}

fn create_gdt_definition() -> Descriptor{
    let null_segment = GDTSegment::null_segment();
    let code_segment = GDTSegment::code_segment();
    let data_segment = GDTSegment::data_segment();

    set_segment_unsafe(0, null_segment);
    set_segment_unsafe(1, code_segment);
    set_segment_unsafe(2, data_segment);

    let descriptor = Descriptor{
        limit: GDT_SIZE,
        base: unsafe {GLOBAL_DESCRIPTOR_TABLE.segments.as_ptr() as u32 }
    };
    return descriptor;
}


fn set_segment_unsafe(index: usize, segment: u64) {
    unsafe {
        GLOBAL_DESCRIPTOR_TABLE.set_segment(index, segment);
    }
}
