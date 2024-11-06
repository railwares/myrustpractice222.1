fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut counts = [0; 5];
    for &bird in arr {
        counts[(bird - 1) as usize] += 1;
    }
    let mut max_count = 0;
    let mut result_bird = 0;
    for i in 0..5 {
        if counts[i] > max_count {
            max_count = counts[i];
            result_bird = (i + 1) as i32;
        } else if counts[i] == max_count && ((i + 1) as i32) < result_bird {
            result_bird = (i + 1) as i32;
        }
    }
    result_bird
}