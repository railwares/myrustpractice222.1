
fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    let shift = ((n % len) + len) % len; // Normalize the shift
    let split_point = (len - shift) as usize;
    let (first_part, second_part) = s.split_at(split_point);
    format!("{}{}", second_part, first_part)
}
fn rotate2(s: &str, n: &isize) -> String {
    rotate(s.to_string(), *n)
}

#[test]
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];


    shifts
        .iter()
        .for_each(|(n, exp)|
            assert_eq!(
                rotate2(s, n),
                exp.to_string()
            )
        );
}
