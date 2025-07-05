use std::collections::HashMap;

pub fn slices_to_map<'a, T: std::hash::Hash + std::cmp::Eq, U>(t: &'a [T], u: &'a [U]) -> HashMap<&'a T, &'a U> {
    let mut map = HashMap::new();

    for i in 0..t.len().min(u.len()) {
        map.insert(&t[i], &u[i]);
    }
    
    map
}