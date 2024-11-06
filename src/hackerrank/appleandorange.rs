fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apples_on_house = apples.iter()
        .map(|&apple| a + apple)
        .filter(|&position| position >= s && position <= t)
        .count();
    let oranges_on_house = oranges.iter()
        .map(|&orange| b + orange)
        .filter(|&position| position >= s && position <= t)
        .count();
    println!("{}", apples_on_house);
    println!("{}", oranges_on_house);
}