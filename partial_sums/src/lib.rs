pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut res = vec![0];
    let mut sum = 0;
    for v in arr {
        sum += v;
        res.push(sum);
    }
    res.into_iter().rev().collect()
}