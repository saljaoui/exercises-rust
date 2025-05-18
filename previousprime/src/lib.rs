pub fn prev_prime(nbr: u64) -> u64  {
    if nbr <= 2 {
        return 0
    }
    for n in (1..nbr).rev() {
        if check(n) {
            return n;
        }
    }
    0
}

fn check(nbr: u64) -> bool {
    for n in 2..nbr {
        if nbr % n == 0 {
        println!("{}, {}", n, nbr);
            return false
        }
    }
    true
}