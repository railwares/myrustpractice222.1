fn draw_tree(triangles: usize) {
    let max_width = 3 * triangles;
    (1..=triangles).for_each(|t| {
        (1..=t).for_each(|line| {
            let stars = 2 * line - 1;
            let fill = (max_width - stars) / 2;
            println!("{}{}", " ".repeat(fill), "*".repeat(stars));
        });
    });
}
#[test]
fn main() {
    let triangles = 6; // Number of triangles for the fir tree
    draw_tree(triangles);
}
