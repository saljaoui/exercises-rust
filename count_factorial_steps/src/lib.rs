pub fn count_factorial_steps(factorial: u64) -> u64 {
    let mut res:u64 = 1;
    if 1 == factorial {
        return 0
    }
    for i in 1..= 20 {
        res *= i;
        if res == factorial {
            return i as u64
        }
    }
    0
}