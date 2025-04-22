pub const INTEGER_8_SIGNED_TYPE: i8 = i8::MAX;
pub const INTEGER_16_SIGNED_TYPE: i16 = i16::MAX;
pub const INTEGER_32_SIGNED_TYPE: i32 = i32::MAX;
pub const INTEGER_64_SIGNED_TYPE: i64 = i64::MAX;
pub const INTEGER_128_SIGNED_TYPE: i128 = i128::MAX;
pub const INTEGER_SIZE_SIGNED_TYPE: isize = isize::MAX;

pub const UNSIGNED_INTEGER_8_TYPE: u8 = u8::MAX;
pub const UNSIGNED_INTEGER_16_TYPE: u16 = u16::MAX;
pub const UNSIGNED_INTEGER_32_TYPE: u32 = u32::MAX;
pub const UNSIGNED_INTEGER_64_TYPE: u64 = u64::MAX;
pub const UNSIGNED_INTEGER_128_TYPE: u128 = u128::MAX;
pub const UNSIGNED_INTEGER_SIZE_TYPE: usize = usize::MAX;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smaller_type_into_higher() {
        let x: i16 = 1;
        let y: i8 = 2;
        assert_ne!(2 + 2, 4);
        assert_eq!(1, 4);
    }
}
