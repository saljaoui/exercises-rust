pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    let mut res: String = String::new();
    for i in s.chars() {
        if i != letter {
            res.push(i)
        }
    }
    res
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    let mut res: String = String::new();
    for i in s.chars() {
        if i.to_ascii_lowercase() != letter.to_ascii_lowercase() {
            res.push(i)
        }
    }
    res
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
    let mut res: String = String::new();
    for i in s.chars() {
        if i.to_ascii_lowercase() == letter.to_ascii_lowercase() {
            if i.is_ascii_lowercase() {
                res.push(i.to_ascii_uppercase())
            } else {
                res.push(i.to_ascii_lowercase())
            }
        } else {
            res.push(i)
        }
    }
    res
}