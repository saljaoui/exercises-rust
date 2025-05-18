pub fn count_factorial_steps(factorial: u64) -> u64 {
    let mut r: u64 = 1;
    for i in 1..factorial {
        if r > factorial {
            break
        }
        r *= i;
        if r == factorial {
            return i 
        }
    }
    0
}