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
// Arrays
pub const INTEGER_ARRAY: [i32; 3] = [0; 3];
pub const BOOLEAN_ARRAY : [bool; 2] = [true, false];
pub const FLOAT_ARRAY : [f32; 3] = [0f32;3];

// Tuples
pub const TUPLE: (char, i32, &str) = ('A', 1, "good");

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

// Ubiquitous enum Types
pub const OPTION: Option<u32> = Some(2);
pub const ABSENT_OPTION: Option<i8> = None;

pub const RESULT: Result<i32, &str> = Ok(-3);
pub const ERROR_RESULT: Result<i32, &str> = Err("Some error message");

// Functions
// Return 'x' divided by 'y'
pub fn div(x: f64, y: f64) -> f64{
    if y == 0.0 {
        // Terminate function and return a value
        return f64::NAN;
    }

    // The last expression in the function body is implicitly returned.
    x / y
}

// Function called just for its side effects, with no return value
// Can also write the return value as '-> ()'.
pub fn show(x: f64) {
    println!("x = {x}");
}

// Methods
pub enum Shape {
    Rectangle {width: f64, height: f64},
    Circle {radius: f64}
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Rectangle {width, height} => width * height,
            Shape::Circle {radius} => std::f64::consts::PI * radius * radius
        }
    }
}

// Function pointer
fn sum (x: i32, y: i32) -> i32 {
    x + y
}

pub const OP: fn(i32,i32) -> i32 = sum;

// Closures

pub fn modify_all(data: &mut [u32], mutator: fn(u32) -> u32){
    for value in data {
        *value = mutator(*value);
    }
}

pub fn add2(v: u32) -> u32 {
    v + 2
}

// *Rough* equivalent to a capturing closure
struct InternalContext<'a> {
    // reference to captured variables
    amount_to_add: &'a u32
}

impl<'a> InternalContext<'a>{
    fn internal_op(&self, y: u32) -> u32 {
        // body of the lambda expression
        y + *self.amount_to_add
    }
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
    fn check_tuple_values_access(){
        assert_eq!(TUPLE.0, 'A');
        assert_eq!(TUPLE.1, 1i32);
        assert_eq!(TUPLE.2, "good");
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

    #[test]
    fn option_has_some(){
        assert_eq!(OPTION.is_some(), true);
        assert_ne!(OPTION.is_none(), true);
    }

    #[test]
    fn absent_option(){
        assert_eq!(ABSENT_OPTION.is_none(), true);
        assert_ne!(ABSENT_OPTION.is_some(), true);
    }

    #[test]
    fn result_is_ok(){
        assert_eq!(RESULT.is_ok(), true);
        assert_ne!(RESULT.is_err(), true);
    }

    #[test]
    fn result_is_error(){
        assert_eq!(ERROR_RESULT.is_err(), true);
        assert_ne!(ERROR_RESULT.is_ok(), true);
    }

    #[test]
    fn check_function_pointer(){
        let op = OP;
        let op1 = OP;
        assert_eq!(op, op1);
    }

    #[test]
    fn check_method_as_argument() {
        let mut data = vec![1, 2, 3];
        modify_all(&mut data, add2);
        assert_eq!(data, vec![3, 4, 5]);
    }
    
    #[test]
    fn check_closures(){
        let amount_to_add = 3;
        let add_n = |y: u32| {
            y + amount_to_add
        };
        let z = add_n(5);
        assert_eq!(z, 8);
    }
    
    #[test]
    fn check_internal_context(){
        let amount_to_add = 3;
        let add_n = InternalContext {
            amount_to_add: &amount_to_add
        };
        let z = add_n.internal_op(5);
        assert_eq!(z, 8);
    }
}
