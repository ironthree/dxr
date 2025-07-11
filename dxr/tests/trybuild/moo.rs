use std::borrow::Cow;

use dxr::{DateTime, TryFromValue, TryToValue};

#[derive(Clone, TryFromValue, TryToValue)]
pub struct Moo<'a> {
    stirr: Cow<'a, str>,
    string: Cow<'a, String>,
    int: Cow<'a, i32>,
    long: Cow<'a, i64>,
    boolean: Cow<'a, bool>,
    double: Cow<'a, f64>,
    tuple: (Cow<'a, String>, Cow<'a, i32>),
    byte_vec: Cow<'a, Vec<u8>>,
    datetime: Cow<'a, DateTime>,
    ref_option: Cow<'a, Option<i32>>,
    option_ref: Option<Cow<'a, i32>>,
    recursive: Box<Cow<'a, Moo<'a>>>,
}

fn main() {}
