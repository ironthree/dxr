use dxr::TryFromValue;

#[derive(TryFromValue)]
pub struct Reference<'a> {
    string: &'a str,
}

fn main() {}
