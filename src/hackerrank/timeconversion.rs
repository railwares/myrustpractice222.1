fn timeConversion(s: &str) -> String {
    let hour = &s[0..2];
    let minute = &s[3..5];
    let second = &s[6..8];
    let period = &s[8..];

    let mut hour = hour.parse::<u32>().unwrap();

    if period == "AM" {
        if hour == 12 {
            hour = 0;
        }
    } else {
        // PM case
        if hour != 12 {
            hour += 12;
        }
    }

    format!("{:02}:{:02}:{:02}", hour, minute, second)
}
