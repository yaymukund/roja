use std::convert::TryFrom;

pub fn usize_to_u16(num: usize) -> u16 {
    u16::try_from(num).expect("number exceeded max u16!")
}
