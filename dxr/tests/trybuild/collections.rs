use std::collections::HashMap;

use chrono::{DateTime, Utc};
use dxr::{FromDXR, ToDXR};

#[derive(ToDXR)]
pub struct ToCollectibles<'a> {
    strings: Vec<String>,
    ints: [i32; 42],
    longs: Vec<i64>,
    booleans: [bool; 69],
    doubles: Vec<f64>,
    tuples: (Vec<String>, [i32; 4]),
    byte_array: [u8; 16],
    datetimes: Vec<DateTime<Utc>>,
    options: [Option<String>; 24],
    map: HashMap<&'a str, [bool; 12]>,
    recursive: [Box<ToCollectibles<'a>>; 2],
}

#[derive(FromDXR, ToDXR)]
pub struct Collectibles {
    strings: Vec<String>,
    ints: Vec<i32>,
    longs: Vec<i64>,
    booleans: [bool; 69],
    doubles: Vec<f64>,
    tuples: (Vec<String>, Vec<i32>),
    datetimes: Vec<DateTime<Utc>>,
    options: Vec<Option<String>>,
    map: HashMap<String, Vec<String>>,
    recursive: Vec<Collectibles>,
}

fn main() {}
