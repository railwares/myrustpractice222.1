fn gradingStudents(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&grade| {
        match grade {
            g if g < 38 => g,
            g if g % 5 >= 3 => g + (5 - g % 5),
            g => g,
        }
    }).collect()
}