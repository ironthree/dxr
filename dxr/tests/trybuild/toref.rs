use dxr::FromDXR;

#[derive(FromDXR)]
pub struct Reference<'a> {
    string: &'a str,
}

fn main() {}
