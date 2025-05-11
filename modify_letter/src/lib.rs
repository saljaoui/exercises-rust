pub fn remove_letter_sensitive(s: &str, letter: char) -> String {

    let mut res: String =  String::new();
    for v in s.chars() {
        if v != letter {
            res.push_str(&v.to_string())
        }
    }
    res
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    let mut res: String =  String::new();
    for v in s.chars() {
        if v != letter && v != letter.to_ascii_uppercase() && v != letter.to_ascii_lowercase(){
            res.push_str(&v.to_string())
        }
    }
    res
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
    let mut res: String =  String::new();
    for v in s.chars() {
        if v == letter || v == letter.to_ascii_uppercase() || v == letter.to_ascii_lowercase(){

            if v.is_ascii_uppercase() {
                res.push_str(&v.to_ascii_lowercase().to_string())
            } else if v.is_ascii_lowercase() {
                res.push_str(&v.to_ascii_uppercase().to_string())
            }
        } else {
            res.push_str(&v.to_string())
        }
    }
    res
}