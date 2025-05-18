pub struct Film {
    pub name: String,
}

pub fn read_film_name(s: &Film) -> String {
    return s.name.clone()
}

pub fn take_film_name(s: Film) -> String {
    return s.name
}