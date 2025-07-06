pub fn min_and_max(nb_1: i32, nb_2: i32, nb_3: i32) -> (i32, i32) {
    let mut vec = vec![nb_1, nb_2, nb_3];
    vec.sort();
    (vec[0],vec[vec.len()-1])
}