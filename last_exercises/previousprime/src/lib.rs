pub fn prev_prime(nbr: u64) -> u64 {
    
    for i in (2..nbr).rev() {
        if check(i) {
            return i;
        }
    }
    0
}
fn check(n: u64) -> bool {
    for i in 2..n {
        if n % i == 0 && n != i {
            return false;
        }
    }
    true
}