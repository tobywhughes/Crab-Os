pub const GDT_ELEMENTS: u8 = 3;
pub const GDT_SIZE: u16 = ((0x0008 * GDT_ELEMENTS as u16) - 1);

#[repr(transparent)]
pub struct GlobalDescriptorTable {
    pub segments: [u64; GDT_ELEMENTS as usize]
}    

impl GlobalDescriptorTable {
    pub fn set_segment(&mut self, index: usize, value: u64) {
        self.segments[index] = value;
    }    

}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Descriptor {
    pub limit: u16,
    pub base: u32
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct GDTSegment {
    pub limit_low: u16,
    pub base_low: u16,
    pub base_mid: u8,
    pub segment_type: u8,
    pub privilege_level: u8,
    pub present_flag: bool,
    pub limit_high: u8,
    pub attributes: u8,
    pub granularity: bool,
    pub base_high: u8
}

impl GDTSegment {
    pub fn u64_representation(self: Self) -> u64 {
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

    pub fn null_segment() -> u64 {
        return 0;
    }

    pub fn code_segment() -> u64 {
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

    pub fn data_segment() -> u64 {
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