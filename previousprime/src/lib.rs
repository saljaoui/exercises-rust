pub fn prev_prime(nbr: u64) -> u64  {
    if nbr < 3 {
        return 0
    }
    for i in (1..nbr).rev() {
        if check(i) {
            return i
        }
    }
    return 0
}

fn check(nbr: u64) -> bool {
    println!("{}", nbr);
    for i in 2..nbr {
        if nbr % i == 0 {
            return false
        }
    }
    true
}
