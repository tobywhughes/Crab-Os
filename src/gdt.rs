
use crate::asm;
use volatile::Volatile;


const NULL_SEGMENT: u64 = 0;

const GDT_ELEMENTS: u8 = 3;
const GDT_SIZE: u16 = ((0x0008 * GDT_ELEMENTS as u16) - 1);


#[repr(transparent)]
pub struct GlobalDescriptorTable {
    segments: [u64; GDT_ELEMENTS as usize]
}

impl GlobalDescriptorTable {
    pub fn set_segment(&mut self, index: usize, value: u64) {
        self.segments[index] = value;
    }


}


use core::ops::{Deref,DerefMut};

use lazy_static::lazy_static;
use spin::Mutex;


static mut GLOBAL_DESCRIPTOR_TABLE: GlobalDescriptorTable = GlobalDescriptorTable {segments: [0_u64; GDT_ELEMENTS as usize]};

fn debug_print_GDT() {        
    unsafe {
        println!(">>> {:?}", GLOBAL_DESCRIPTOR_TABLE.segments);
    }
}

// lazy_static! {
//     static ref GLOBAL_DESCRIPTOR_TABLE: Mutex<&'static GlobalDescriptorTable> =  Mutex::new(&GlobalDescriptorTable {segments: [0_u64; GDT_ELEMENTS as usize]});
// }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Descriptor {
    pub limit: u16,
    pub base: u32
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct GDTSegment {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    segment_type: u8,
    privilege_level: u8,
    present_flag: bool,
    limit_high: u8,
    attributes: u8,
    granularity: bool,
    base_high: u8
}

impl GDTSegment {
    fn u64_representation(self: Self) -> u64 {
        return (
            ((self.base_low as u64) << 48) |
            ((self.limit_low as u64) << 32) |
            ((self.base_high as u64) << 24) |
            ((self.granularity as u64) << 23) |
            (((self.attributes & 0x07) as u64) << 20) |
            (((self.limit_high & 0x0F) as u64) << 16) |
            ((self.present_flag as u64) << 15) |
            (((self.privilege_level & 0x03) as u64) << 13) |
            (((self.segment_type & 0x1F) as u64) << 8) |
            (self.base_mid as u64)
        ) as u64
    }

    fn code_segment() -> u64 {
        let segment = GDTSegment {
            limit_low: 0xFFFF,
            base_low: 0,
            base_mid: 0x00,
            segment_type: 0b00011010,
            privilege_level: 0x00,
            present_flag: true,
            limit_high: 0x0F,
            attributes: 0b00000100,
            granularity: true,
            base_high: 0x00
        };
        return segment.u64_representation();
    }

    fn data_segment() -> u64 {
        let segment = GDTSegment {
            limit_low: 0xFFFF,
            base_low: 0,
            base_mid: 0x00,
            segment_type: 0b00010010,
            privilege_level: 0x00,
            present_flag: true,
            limit_high: 0x0F,
            attributes: 0b00000100,
            granularity: true,
            base_high: 0x00
        };
        return segment.u64_representation();
    }
}

use crate::println;

pub fn enter_protected_mode() {
    println!("ENTERING PROTECTED MODE...\n");
    asm::disable_interrupts();
    let GDT_DESCRIPTOR: Descriptor = create_gdt_definition();
    println!("GDT DESCRIPTOR PRE:  {:?}", GDT_DESCRIPTOR);
    asm::load_global_descriptor_table(&GDT_DESCRIPTOR);
    let temp = asm::get_global_descriptor_table();
    println!("GDT DESCRIPTOR POST: {:?}\n\n", temp);

}

pub fn create_gdt_definition() -> Descriptor{
    let code_segment = GDTSegment::code_segment();
    let data_segment = GDTSegment::data_segment();

    unsafe {
        GLOBAL_DESCRIPTOR_TABLE.set_segment(0, NULL_SEGMENT);
        GLOBAL_DESCRIPTOR_TABLE.set_segment(1, code_segment);
        GLOBAL_DESCRIPTOR_TABLE.set_segment(2, data_segment);
    }

    let descriptor = Descriptor{
        limit: GDT_SIZE,
        base: unsafe {GLOBAL_DESCRIPTOR_TABLE.segments.as_ptr() as u32 }
    };
    return descriptor;
}

#[test_case]
fn test_correctly_built_gdt_segment_returns_correct_u64() {
    let segment = GDTSegment {
        limit_low: 0,
        base_low: 0xFFFF,
        base_mid: 0xFF,
        segment_type: 0b00011111,
        privilege_level: 0b00000011,
        present_flag: true,
        limit_high: 0b00001111,
        attributes: 0b00000111,
        granularity: true,
        base_high: 0xFF
    };

    let expected_result: u64 = 0xFFFF0000FFFFFFFF;
    let actual_result: u64 = segment.u64_representation();
    assert_eq!(expected_result, actual_result);
}