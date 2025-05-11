use std::collections::HashMap;
pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut hashmap = HashMap::new();
    let vec: Vec<&str> =  words.split(" ").collect();
    for v in vec {
    let mut res: String = String::new(); 
    for i in v.chars() {
        if i != '!' &&  i != ',' && i != ';' && i != '.' && i != '―' && i != '/' && i != '“' && i != '\n' && i != '”' {
            res.push_str(&i.to_ascii_lowercase().to_string())
        }
    }
    if !res.is_empty() {
        let count = hashmap.entry(res).or_insert(0);
        *count += 1
    }

    }
    hashmap
}