pub fn lucas_number(n: u32) -> u32 {
    let mut res: u32 = 0;
    let mut a: u32 = 2;
    let mut b: u32 = 1;
    for i in 2..=n {
        res = a + b;
        a = b;
        b = res;
    }
    res
}