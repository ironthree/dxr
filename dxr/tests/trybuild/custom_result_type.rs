use dxr::{TryFromValue, TryToValue};

type Result<T> = std::result::Result<T, ()>;

#[derive(TryFromValue, TryToValue)]
pub struct Array {
    array: [i32; 4],
}

fn main() {}
