pub fn scytale_decoder(s: String, letters_per_turn: u32) -> Option<String> {
    let len = (s.len() as f64 / letters_per_turn as f64).ceil() as usize;
    for i in 0..letters_per_turn {
        for f in 0..len {
        }
    }
    println!("{:?}", len);
    Some("ok".to_string())
}