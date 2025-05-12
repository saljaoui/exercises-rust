pub fn insertion_sort(slice: &mut [i32], steps: usize) {
    for i in 0..=steps {
        for j in 0..i+1 {
            if slice[i] < slice[j] {
                slice.swap(j, i);
            }
        }
    }
}