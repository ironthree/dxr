use std::rc::Rc;
use std::sync::Arc;

use dxr::{DateTime, TryFromValue, TryToValue};

#[derive(TryFromValue, TryToValue)]
pub struct Ownership {
    string: String,
    int: i32,
    long: i64,
    boolean: bool,
    double: f64,
    tuple: (String, i32, i64, bool, f64, Vec<u8>),
    byte_vec: Vec<u8>,
    datetime: DateTime,
    option: Option<i32>,
    recursive: Box<Ownership>,
    counted: Rc<String>,
    atomically: Arc<Vec<u8>>,
}

fn main() {}
