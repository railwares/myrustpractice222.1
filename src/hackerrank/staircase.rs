fn staircase(n: i32) {
    for i in 1..=n {
        let space = " ".repeat((n - i) as usize);
        let hash = "#".repeat(i as usize);
        println!("{}{}", space, hash);
    }
}