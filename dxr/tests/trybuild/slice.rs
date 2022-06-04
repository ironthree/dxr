use dxr::{FromDXR, ToDXR};

#[derive(FromDXR, ToDXR)]
pub struct Slice {
    slice: [i32],
}

fn main() {}
