use std::collections::HashMap;

pub fn smallest(h: HashMap<&str, i32>) -> i32 {
    let mut res: i32 = i32::MAX;
    for (key, val) in h {
        if res >= val {
            res = val
        }
    }
    res
}