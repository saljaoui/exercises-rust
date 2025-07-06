pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    for ii in 1..=i {
        let mut collect = String::new();
        collect.push_str(&lop(" ", ii));
        collect.push_str(&lop(&v, ii));
        res.push(collect);
    }

    for ii in (1..i).rev() {
        let mut collect = String::new();
        collect.push_str(&lop(" ", ii));
        collect.push_str(&lop(&v, ii));
        res.push(collect);
    }
    
    res
}

fn lop(v: &str, i: u32) -> String {
    let mut res: String = String::new();
    for ii in 0..i {
        res.push_str(v);
    }
    res
}