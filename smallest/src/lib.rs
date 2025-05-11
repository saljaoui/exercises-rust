use std::collections::HashMap;
use std::i32::MAX;

pub fn smallest(h: HashMap<&str, i32>) -> i32 {
    let mut min: i32 = MAX;
    for (_, v) in h {
        if v < min {
            min = v;
        }
    }
    min
}