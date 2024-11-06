fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    let mut count = 0;
    let mut sum: i32 = s.iter().take(m as usize).sum();

    if sum == d {
        count += 1;
    }

    for i in m..s.len() as i32 {
        sum = sum - s[(i - m) as usize] + s[i as usize];
        if sum == d {
            count += 1;
        }
    }

    count
}