fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    let max_height = match candles.iter().max() {
        Some(&max) => max,
        None => return 0,
    };
    candles.iter().filter(|&&height| height == max_height).count() as i32
}