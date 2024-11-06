fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    let mut highest = scores[0];
    let mut lowest = scores[0];
    let mut high_count = 0;
    let mut low_count = 0;

    for &score in &scores[1..] {
        if score > highest {
            highest = score;
            high_count += 1;
        }
        if score < lowest {
            lowest = score;
            low_count += 1;
        }
    }

    vec![high_count, low_count]
}