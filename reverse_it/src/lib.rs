pub fn reverse_it(v: i32) -> String {
    let mut res: String = String::new();
    let chars: String = v.abs().to_string().chars().rev().collect();
        println!("{:?}", chars);

        if v < 0 {
            res.push_str("-");
        }
        res.push_str(&chars);
        res.push_str(&v.abs().to_string());

    res
}