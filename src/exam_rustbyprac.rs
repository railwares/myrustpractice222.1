// Remove a line in the code to make it compile
#[test]
fn task1() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x;
    // remove: x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!";

    println!("Success!");
}

#[test]
fn task2() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3,2]); //[3,2]

    println!("Success!");
}

#[test]
// Fill the blanks to make it work
fn task3() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255); // 127, 255

    println!("Success!");
}

#[test]
fn task4() {
    // Make it work, don't modify `implicitly_ret_unit` !
        let v1: () = ();                // let v1:, assert eq v -> v1

        let v = (2, 3);
        assert_eq!(v1, implicitly_ret_unit());

        println!("Success!");
    }

    fn implicitly_ret_unit() {
        println!("I will return a ()");
    }

    // Don't use this one
    fn explicitly_ret_unit() -> () {
        println!("I will return a ()");
    }

#[test]
fn task5() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y          //remove ; for x+y return
}

#[test]
fn task6() {
    print();
}

// Replace i32 with another type
fn print() -> () {  //removed i32
    println!("Success!");
}

#[test]
fn task7() {  //replace can be used to replace substring
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats"); //+replace

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

#[test]
fn task8() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");   //s1->&s1, 0..1

    let h1 = &s1[3..6]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");  //5->6

    println!("Success!");
}

#[test]
fn task9() {
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];

    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12); //__->12

    println!("Success!");
}

#[test]
fn task10() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16);  //8 -> 16

    println!("Success!");
}

#[test]
fn task11() {
    fn main() {
        let (x, y, z);

        // Fill the blank
        (y, z, x) = (1, 2, 3); //__-> (y, z, x)

        assert_eq!(x, 3);
        assert_eq!(y, 1);
        assert_eq!(z, 2);

        println!("Success!");
    }
}


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
#[test]
fn task12() {  // Fix the error and fill the blanks
    let v = Point(0, 127, 255);
    check_color(v);
    println!("Success!");
}

fn check_color(p: Point) { //Color -> Point
    let Point(x, _, _) = p;  //+Point
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);   //+p.2
}

// Fill in the blank and fix the errors
#[derive(Debug)]  //+#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[test]
fn task13() {
    let msgs: [Message; 3] = [  //__->[Message; 3]
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
}

fn show_message(msg: Message) {
    println!("{:?}", msg);  //+ :?
}

#[test]
fn task14() {
    for n in 1..=99 { // modify this line to make the code work
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
}

#[test]
fn task15() {
    let mut n = 0;
    for i in 0..=100 {
        if n != 66 {
            n+=1;
            continue;
        }

        break;
    }

    assert_eq!(n, 66);

    println!("Success!");
}

#[test]
fn task16() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break counter * 2
        }
    };

    assert_eq!(result, 20);

    println!("Success!");
}

enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[test]
fn task17() {
    let msgs = [
        Message2::Quit,
        Message2::Move{x:1, y:3},
        Message2::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message2(msg)
    }

    println!("Success!");
}

fn show_message2(msg: Message2) {
    match msg {
        Message2::Move {x: a, y: b} => { // match  Message::Move {x:a, y:b}
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message2::ChangeColor(_, g, b) => {
            assert_eq!(g, 255);  //255, 0
            assert_eq!(b, 0);
        }
        _ => println!("no data in these variants")  //__->_
    }
}


#[test]
fn task18() {
    let age = Some(30);
    if let Some(age) = age { // Create a new variable with the same name as previous `age`
        assert_eq!(age, 30);// remove Some()
    } // The new variable `age` goes out of scope here

    match age {
        // Match can also introduce a new shadowed variable
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
}

#[test]
fn task19() {}
fn match_number(n: i32) {
    match n {
        // Match a single value
        1 => println!("One!"),
        // Fill in the blank with `|`, DON'T use `..` or `..=`
        2 | 3 | 4 | 5 => println!("match 2 -> 5"), // 2 | 3 | 4 | 5
        // Match an inclusive range
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => {
            println!("match -infinite -> 0 or 11 -> +infinite")
        }
    }
}

#[test]
fn task20() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first,..,last) => {  //(first,..,last)
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }

    println!("Success!");
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Complete the area method which return the area of a Rectangle.
    fn area (&self) -> u32 {     // (&self) -> u32 // self.width * self.height
        self.width * self.height
    }
}

fn task21() {
    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);

    println!("Success!");
}

#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // 1. Implement an associated function `new`,
    // 2. It will return a TrafficLight contains color "red"
    // 3. Must use `Self`, DONT use `TrafficLight` in fn signatures or body
    // -> Self {
    // Self  {
    // color: "red".to_string()
    //  }
    // }
    pub fn new() -> Self {
        Self {
            color: "red".to_string()
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}
#[test]
fn task22() {
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");

    println!("Success!");
}


// Implement struct Point to make it work.

struct Point2<A> {    //create a struct for point2
    x: A,
    y: A
}

#[test]
fn task23() {
    let integer = Point2 { x: 5, y: 10 };
    let float = Point2 { x: 1.0, y: 4.0 };

    println!("Success!");
}

// Fill in the two impl blocks to make the code work.
// DON'T modify the code in `main`.
trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String;
}

struct Student {}
impl Hello for Student {
    fn say_something(&self) -> String {
        String::from("I'm a good student")
    }
}
struct Teacher {}
impl Hello for Teacher {
    fn say_hi(&self) -> String {
        String::from("Hi, I'm your new teacher")
    }

    fn say_something(&self) -> String {
        String::from("I'm not a bad teacher")
    }
}
#[test]
fn task24() {
    let s = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");

    println!("Success!");
}


// Implement `fn summary` to make the code work.
// Fix the errors without removing any code line
trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}
#[test]
fn task25() {
    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };

    summary(&post);
    summary(&weibo);

    println!("{:?}", post);
    println!("{:?}", weibo);
}
fn summary(t: &impl Summary) {   //add & to errors and
    let _ = t.summarize();
}
// Implement `fn summary` below.

trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String{
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String{
        "swan swan".to_string()
    }
}
#[test]
fn task26() {
    // FILL in the blank.
    let duck = Duck;
    duck.swim();

    let bird = hatch_a_bird(2);
    // This bird has forgotten how to swim, so below line will cause an error.
    // bird.swim();
    // But it can quak.
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    // This bird has forgotten how to fly, so below line will cause an error.
    // bird.fly();
    // But it can quak too.
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");
}

// IMPLEMENT this function.
fn hatch_a_bird(species: u8) ->Box<dyn Bird> {
    if species == 1 {
        Box::new(Swan{})
    } else {
        Box::new(Duck{})
    }
}

// FILL in the blanks and FIX errors
// 1. Don't use `to_string()`
// 2. Don't add/remove any code line
#[test]
    // FILL in the blanks
    fn task27() {
        let mut s = String::from("hello, world");

        let slice1: &str = &s;
        assert_eq!(slice1, "hello, world");

        let slice2 = &s[0..5];
        assert_eq!(slice2, "hello");

        let slice3: &mut String = &mut s;
        slice3.push('!');
        assert_eq!(slice3, "hello, world!");

        println!("Success!");
    }


#[test]
fn task28() {  // FILL in the blank and FIX errors
    let s = String::from("hello, 世界");
    let slice1 = &s[0..1]; //tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10]; // Tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(slice2, "世");

    // Iterate through all chars in s
    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }

    println!("Success!");
}

#[test]
fn task29() {
    assert_eq!(1000 as u16, 1000);

    assert_eq!(255 as u8, 255);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    assert_eq!(-1_i8 as u8, 255);

    // Since Rust 1.45, the `as` keyword performs a *saturating cast*
    // when casting from float to int. If the floating point value exceeds
    // the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.
    assert_eq!(300.1_f32 as u8, 255);
    assert_eq!(-100.1_f32 as u8, 0);


    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}

#[test]
fn task30() {
    /* Fill in the blanks to make it print:
  Hello world, I am
  Sunface!
  */
    print!("hello world, ");
    println!("I am");
    print!("Sunface!");
}

#[test]
fn task31() {
    // Left align
    println!("Hello {:<5}!", "x"); // => Hello x    !
    // Right align
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
    // Center align
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");

    // Left align, pad with '&'
    assert_eq!(format!("Hello {:&<5}!", "x"), "Hello x&&&&!");

    println!("Success!");
}
