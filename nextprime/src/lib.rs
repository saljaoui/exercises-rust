pub fn next_prime(nbr: u64) -> u64 {
    if nbr < 3 {
        return 2
    }

    for i in nbr..1000000 {
        if check(i) {
            return i;
        }
    }
    0
}

fn check(nbr: u64) -> bool {
    for n in 2..nbr {
        if nbr % n == 0 {
            return false
        }
    }
    return true
}