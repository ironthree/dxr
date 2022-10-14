use dxr::{TryFromValue, TryToValue};

#[derive(TryFromValue, TryToValue)]
pub struct Slice {
    slice: [i32],
}

fn main() {}
