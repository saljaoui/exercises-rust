pub fn count_factorial_steps(factorial: u64) -> u64 {
    let mut res: u64 = 1;
    if factorial == 1 {
        return 0;
    }
    
    for i in 1..u64::MAX {
        res *= i;
        if res == factorial {
            return i;
        } else if res > factorial {
            return 0;
        }
    }
    0
}