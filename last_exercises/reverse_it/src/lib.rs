pub fn reverse_it(v: i32) -> String {
    let num = v.abs();
    let rev: String = num.to_string().chars().rev().collect();
    let mut res = String::new();
    if v < 0 {
        res.push('-')
    }
    res.push_str(&rev);
    res.push_str(&num.to_string());
    res
}