pub fn scytale_decoder(message: String, i: u32) -> Option<String> {
    let lines = (message.len() as f64 / i as f64).ceil() as usize ;
    // println!("{:?}", lines);

    // let mut res: String = String::new();
    if message == "" || i == 0 {
        return None
    }

    let mut vec: Vec<String> = vec![];

    for s in 0..lines {
        let start = s * i as usize;
        let end = (s + 1) * i as usize;
        if message.len() > end {
        // println!("{:?}", &message[start..end]);
        vec.push(message[start..end].to_string());
        } else {
            let mut space: String = String::new();
            for s in 0..( end - message.len()) {
            space.push(' ');
            }
            // println!("{:?}", &message[start..message.len()]);
            vec.push(message[start..message.len()].to_string() + &space);
        }
    }

            println!("{:?}", vec);
let ok: String = read_scytale_cipher(vec, i);
Some(ok.trim().to_string())

}


fn read_scytale_cipher(scytale: Vec<String>, i: u32) -> String {
    let mut res: String = String::new();
for n in 0..i {
    for s in &scytale {
        let start = n as usize;
        let end = (n + 1) as usize;
        res.push_str(&s[start..end]);
    }
} 
res
}