fn plusMinus(arr: &[i32]) {
    let mut positives = 0;
    let mut negatives = 0;
    let mut zeros = 0;
    let n = arr.len() as f32;

    for &numbers in arr {
        match numbers {
            x if x > 0 => positives += 1,
            x if x < 0 => negatives += 1,
            _ => zeros += 1,
        }
    }

    println!("{:.6}", positives as f32 / n);
    println!("{:.6}", negatives as f32 / n);
    println!("{:.6}", zeros as f32 / n);
}