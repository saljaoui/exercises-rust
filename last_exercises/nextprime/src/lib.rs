pub fn next_prime(nbr: u64) -> u64 {
    let max = u64::MAX;
    if nbr < 2 {
        return 2;
    }

    for i in nbr..max {
        if check(i) {
            return i;
        }
    }
    2
}

fn check(n: u64) -> bool {
    for i in 2..100 {
        if n % i == 0 && n != i {
            return false;
        }
    }
    true
}