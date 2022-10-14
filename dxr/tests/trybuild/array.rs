use dxr::{TryFromValue, TryToValue};

#[derive(TryFromValue, TryToValue)]
pub struct Array {
    array: [i32; 4],
}

fn main() {}
