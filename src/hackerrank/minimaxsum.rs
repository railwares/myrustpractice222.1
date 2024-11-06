fn miniMaxSum(arr: &[i32]) {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    let min_sum: i64 = sorted_arr.iter().take(4).map(|&x| x as i64).sum();
    let max_sum: i64 = sorted_arr.iter().skip(1).map(|&x| x as i64).sum();
    println!("{} {}", min_sum, max_sum);
}