use std::collections::{HashMap, HashSet};

// Fundamental types
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

pub const BOOLEAN : bool = false;

pub const FLOAT_32: f32 = f32::MAX;
pub const FLOAT_64: f64 = f64::MAX;

pub const CHARTER : char = 'a';

pub const INTEGER_FROM_CHAR: char = char::from_u32(1).unwrap();

// Aggregate types
pub const INTEGER_ARRAY : [i32; 3] = [0;3];
pub const BOOLEAN_ARRAY : [bool; 2] = [true, false];

pub const FLOAT_ARRAY : [f32; 3] = [0f32;3];

pub struct TextMatch(usize, String);

pub enum HttpResultCode {
    Ok = 200,
    NotFound = 404,
    Teapot = 418
}

pub struct Job();
pub struct CpuId();
pub enum SchedulerState {
    Inert,
    Pending(HashSet<Job>),
    Running(HashMap<CpuId, Vec<Job>>)
}

// Undesired behavior
pub struct RgbColor(u8, u8, u8);
pub struct DisplayProps {
    pub x: u32 ,
    pub y: u32 ,
    pub monochrome: bool,
    pub fg_color: RgbColor // <--- must be 0, 0, 0 if 'monochrome' is true
}

pub enum Color {
    Monochrome,
    Foreground(RgbColor),
}

pub struct DisplayPropsGoodVar {
    pub x: u32,
    pub y: u32,
    pub color: Color
}

#[cfg(test)]
mod tests {
    use super::*;

    // this test is my example. May be here can be better example
    #[test]
    fn higher_type_into_smaller() {
        let _x = 15i8; // Integer literal with type suffix
        let x: i16 = i8::MAX as i16;
        // let y: i8 = x; <-- mismatched types
        let _y : i8 = x as i8; // returned value: -1

        // Not from book. It's tip from IDE
        let y : Result<i8, _> = x.try_into();
        assert!(y.is_ok());
    }

    #[test]
    fn check_struct_instance_property_access(){
        // Construct by providing the content in order
        let m = TextMatch(12, "needle".to_owned());
        assert_eq!(m.0, 12);
    }

    #[test]
    fn enum_as_integer(){
        let status_code = HttpResultCode::NotFound;
        assert_eq!(status_code as i32, 404);
    }

    #[test]
    fn match_status_code(){
        let status_code = HttpResultCode::Ok;
        let msg = match status_code {
            HttpResultCode::NotFound => "not_found",
            HttpResultCode::Teapot => todo!(), // Mechanism that indicate result of matching for Teapot - need to do!
            HttpResultCode::Ok => "ok",
        };
        assert_eq!(msg, "ok");
    }


}
