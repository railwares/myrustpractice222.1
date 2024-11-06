fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    let mut count = 0;
    for i in a[a.len() - 1]..=b[0] {
        if a.iter().all(|x| i % x == 0) && b.iter().all(|x| x % i == 0) {
            count += 1;
        }
    }
    return count;
}