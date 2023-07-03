use dxr::chrono::NaiveDateTime;
use dxr::TryToValue;

#[derive(TryToValue)]
pub struct Appendix<'a> {
    string: &'a str,
    int: &'a i32,
    long: &'a i64,
    boolean: &'a bool,
    double: &'a f64,
    tuple: (&'a str, &'a i32),
    byte_slice: &'a [u8],
    byte_vec_ref: &'a Vec<u8>,
    datetime: &'a NaiveDateTime,
    ref_option: &'a Option<i32>,
    option_ref: Option<&'a i32>,
    recursive: &'a Appendix<'a>,
}

fn main() {}
