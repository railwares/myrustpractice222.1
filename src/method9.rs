
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Complete the area method which return the area of a Rectangle.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
#[test]
fn task1() {
    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);

    println!("Success!");
}


// Only fill in the blanks, DON'T remove any line!
#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(&self)  {
        println!("the current state is {}", self.color); //self will take the ownership of current struct instance, however, &self will only borrow a reference from the instance.
    }
}
#[test]
fn task2() {
    let light = TrafficLight{
        color: "red".to_owned(),
    };
    // Don't take the ownership of `light` here.
    light.show_state();
    // ... Otherwise, there will be an error below
    println!("{:?}", light);
}

struct TrafficLight2 {
    color: String,
}

impl TrafficLight2 {
    // Using `Self` to fill in the blank.
    pub fn show_state(self)  {
        println!("the current state is {}", self.color);
    }

    // Fill in the blank, DON'T use any variants of `Self`.
    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }
}
fn task3() {
    println!("Success!");
}

#[derive(Debug)]
struct TrafficLight3 {
    color: String,
}

impl TrafficLight3 {
    // 1. Implement an associated function `new`,
    // 2. It will return a TrafficLight contains color "red"
    // 3. Must use `Self`, DONT use `TrafficLight` in fn signatures or body
    pub fn new() -> Self {
        Self {
            color: "red".to_string(),
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}
#[test]
fn task4() {
    let light = TrafficLight3::new();
    assert_eq!(light.get_state(), "red");

    println!("Success!");
}

// All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl.
// We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with.

struct Rectangle2 {  //Each struct is allowed to have multiple impl blocks.
    width: u32,
    height: u32,
}

// Using multiple `impl` blocks to rewrite the code below.
impl Rectangle2 {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle2 {
    fn can_hold(&self, other: &Rectangle2) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[test]
fn task5() {
    println!("Success!");
}

#[derive(Debug)]  //We can also implement methods for enums.
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    fn color(&self) -> String {
        match self {
            TrafficLightColor::Red => String::from("red"),
            TrafficLightColor::Yellow => String::from("yellow"),
            TrafficLightColor::Green => String::from("green"),
        }
    }
}
#[test]
fn task6() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}