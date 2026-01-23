pub struct Alu;

impl Alu {
    pub fn add(data1: u32, data2: u32) -> u32 {
        data1.wrapping_add(data2)
    }

    pub fn add_signed(data1: u32, data2: i32) -> u32 {
        data1.wrapping_add_signed(data2)
    }

    pub fn sub(data1: u32, data2: u32) -> u32 {
        data1.wrapping_sub(data2)
    }

    pub fn xor(data1: u32, data2: u32) -> u32 {
        data1 ^ data2
    }

    pub fn or(data1: u32, data2: u32) -> u32 {
        data1 | data2
    }

    pub fn and(data1: u32, data2: u32) -> u32 {
        data1 & data2
    }

    pub fn shl_logic(data: u32, shift: u32) -> u32 {
        data << (shift % 32)
    }

    pub fn shr_logic(data: u32, shift: u32) -> u32 {
        data >> (shift % 32)
    }

    pub fn shr_ar(data: i32, shift: u32) -> u32 {
        (data >> (shift % 32)) as u32
    }

    pub fn set_less_than(data: i32, cmp_data: i32) -> u32 {
        (data < cmp_data).into()
    }

    pub fn set_less_than_unsigned(data: u32, cmp_data: u32) -> u32 {
        (data < cmp_data).into()
    }

    pub fn mul(data1: u32, data2: u32) -> u32 {
        (data1 as i32 as i64)
            .wrapping_mul(data2 as i32 as i64) as u32
    }

    pub fn mulh(data1: u32, data2: u32) -> u32 {
        ((data1 as i32 as i64)
            .wrapping_mul(data2 as i32 as i64) >> 32) as u32
    }

    pub fn mulh_unsigned(data1: u32, data2: u32) -> u32 {
        ((data1 as u64)
            .wrapping_mul(data2 as u64) >> 32) as u32
    }
    
    pub fn mulh_signed_unsigned(data1: u32, data2: u32) -> u32 {
        ((data1 as i32 as i64)
            .wrapping_mul(data2 as i64) >> 32) as u32
    }
    
    pub fn div(data1: u32, data2: u32) -> u32 {
        if data2 == 0 {
            u32::MAX
        } else {
            (data1 as i32)
                .wrapping_div(data2 as i32) as u32
        }
    }

    pub fn div_unsigned(data1: u32, data2: u32) -> u32 {
        if data2 == 0 {
            u32::MAX
        } else {
            data1.wrapping_div(data2) 
        }
    }

    pub fn rem(data1: u32, data2: u32) -> u32 {
        if data2 == 0 {
            data1
        } else {
            (data1 as i32)
                .wrapping_rem(data2 as i32) as u32
        }
    }

    pub fn rem_unsigned(data1: u32, data2: u32) -> u32 {
        if data2 == 0 {
            data1
        } else {
            data1.wrapping_rem(data2) 
        }
    }
}
