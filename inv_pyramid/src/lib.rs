pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut string: String = String::new();
    
    if 0 == i {
        return res;
    }

    for index in 0..i {
        string.push_str(&draw(" ", index));
        string.push_str(&draw(&v, index));

        if !string.is_empty() {
            res.push(string.clone());
            string.clear();
        }
    }

    for index in (0..res.len() - 1).rev() {
        res.push(res[index].clone());
    }

    res
}

fn draw(str: &str, index: u32) -> String {
    let mut string: String = String::new();
    for _ in 0..index + 1 {
        string.push_str(str);
    }
    string
}