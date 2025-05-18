pub fn lucas_number(n: u32) -> u32 {
    let mut r: u32 = 2;
    let mut a = 2;
    let mut b = 1;

    for _i in 1..n {
        r = a + b;
        b = r;
        a = b;
    }
    
    return r
}