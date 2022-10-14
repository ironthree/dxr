use dxr::{TryFromValue, TryToValue};

#[derive(TryFromValue, TryToValue)]
pub struct Recursive {
    recursive: Box<Recursive>,
}

fn main() {}
