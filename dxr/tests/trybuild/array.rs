use dxr::{FromDXR, ToDXR};

#[derive(FromDXR, ToDXR)]
pub struct Array {
    array: [i32; 4],
}

fn main() {}
