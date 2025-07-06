use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut hash_map: HashMap<String, u32> = HashMap::new();
    let split: Vec<&str> = words.split_whitespace().collect();
    for s in split {
        let correct_word = correct_word(s);
        if correct_word != "" {
            *hash_map.entry(correct_word.to_string()).or_insert(0) += 1
        }
    }
   
    hash_map
}

fn correct_word(word: &str) -> String {
    let mut res: String = String::new();
    let chars: Vec<char> = word.chars().collect();

    for (i, &w) in chars.iter().enumerate() {
        if w.is_alphanumeric() {
            res.push(w.to_ascii_lowercase());
        } else if i > 0 && i < chars.len() - 1 && chars[i + 1].is_alphanumeric() && chars[i - 1].is_alphanumeric() {
            res.push(w);
        }
    }
    res
}