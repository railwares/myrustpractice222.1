use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut occupied_points = HashSet::new();

    for rectangle in xs {
        // Loop over each point within the rectangle's bounds
        for x in rectangle.a.x..rectangle.b.x {
            for y in rectangle.b.y..rectangle.a.y {
                occupied_points.insert(Point { x, y });
            }
        }
    }
    occupied_points.len() as i32
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
    println!("Passed!");
}
#[test]
fn main() {
    area_occupied_test();
}