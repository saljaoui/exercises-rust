pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    let mut string: String = String::new();

    for index1 in 0..i {
        for _ in 0..index1 + 1 {
            string.push_str(" ");
        }
        for _ in 0..index1 + 1 {
            string.push_str(&v);
        }

        if !string.is_empty() {
            res.push(string.clone());
            string.clear();
        }
    }

    for index1 in (0..i).rev() {
        for _ in 0..index1 {
            string.push_str(" ");
        }
        for _ in 0..index1 {
            string.push_str(&v);
        }
        if !string.is_empty() {
            res.push(string.clone());
            string.clear();
        }
    }

    res
}
