pub fn parts_sums(arr: &[u64]) -> Vec<u64>{
    let res: Vec<u64> = Vec::new();
    for i in 0..arr.len() {
        let soufian = 0;
        for ii in i..arr.len() {
            soufian += arr[ii]
        }
        res.push(soufian)
    }
    res
}