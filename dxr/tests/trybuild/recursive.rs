use dxr::{FromDXR, ToDXR};

#[derive(FromDXR, ToDXR)]
pub struct Recursive {
    recursive: Box<Recursive>,
}

fn main() {}
