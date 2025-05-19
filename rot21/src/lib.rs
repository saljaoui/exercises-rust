pub fn rot21(input: &str) -> String {
    let res: String = String::new();
    for i in input.chars() {
        if i.is_lowercase() {
            println!("{:?}", char::from_u32((i as u32  + 21)).expect("REASON").to_string())
            // res.push_str((i as u32  + 21))
        }
    }
    todo!()
}