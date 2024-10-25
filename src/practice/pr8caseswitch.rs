fn invert_the_case(s: String) -> String {
    let mut result = String::with_capacity(s.len());

    for c in s.chars() {
        if c.is_lowercase() {
            result.push(c.to_uppercase().next().unwrap());
        } else if c.is_uppercase() {
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }

    result
}

#[test]
fn test() {
    let data =
        [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];


    data
        .iter()
        .for_each(|(a, b)| {
            assert_eq!(
                invert_the_case(a.to_string()),
                b.to_string()
            );
            assert_eq!(
                invert_the_case(b.to_string()),
                a.to_string()
            );
        });
}
