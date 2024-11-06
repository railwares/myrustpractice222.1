fn pageCount(n: i32, p: i32) -> i32 {
    let forward = p / 2;
    let mut backward = (n - p) / 2;
    if (n - p) == 1 && n % 2 == 0 {
        backward = 1;
    }
    std::cmp::min(forward, backward)
}