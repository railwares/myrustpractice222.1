
//1.Numbers
//Tips: If we don't explicitly assign a type to a variable, then the compiler will infer one for us.
use std::ops::{Range, RangeInclusive};
use std::mem::size_of_val;
#[test]
#[allow(unused_variables)]
fn task1() { // Remove something to make it work
    let x = 5;
    let mut y: u32 = 5;

    y = x;

    let _z = 10; // Type of z ?

    println!("Success!");
}

#[test]
fn task2() { // Fill the blank
    let _v: u16 = 38_u8 as u16;

    println!("Success!");
}

#[test]
fn task3() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn task4() {
        assert_eq!(i8::MAX, 127);
        assert_eq!(u8::MAX, 255);

        println!("Success!");
}

#[test]
fn task5() {
    let v1 = 247_u8 + 8;
    let v2 = i8::checked_add(118, 8).unwrap();
    println!("{},{}",v1,v2);
}

#[test]
fn task6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);
    println!("Success!");
}

#[test]
#[allow(unused_variables)]
fn task7() {
    let x = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn type_of2<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn task8() {
    assert!(0.1_f32+0.2==0.3_f32);
    println!("Success!");
}

// Two goals: 1. Modify assert! to make it work 2. Make println! output: 97 - 122
#[test]
fn task9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c as u8);
    }
}

#[test]
fn task10() {
    assert_eq!(1..5, Range{ start: 1, end: 5 });
    assert_eq!(1..=5, RangeInclusive::new(1, 5));

    println!("Success!");
}

#[test]
// Fill the blanks and fix the errors
fn task11() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == 3);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9.6 / 3.2 == 3.0); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

// Char, Bool and Unit
#[test]
fn task1_2() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4);

    println!("Success!");
}

#[test]
fn task2_2() {
        let c1 = '中';
        print_char(c1);
}

    fn print_char(c : char) {
        println!("{}", c);
}

#[test]
fn task3_2() {
    let f: bool = false;

    let _t = true;
    if !f {
        println!("Success!");
    }
}

#[test]
fn task4_2() {
        let f = true;
        let t = true;
        assert_eq!(t, f);

        println!("Success!");
}

#[test]
fn task5_2() {
    let v2: () = ();

    let _v = (2, 3);
    assert_eq!(v2, implicitly_ret_unit());

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
fn task6_2() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}
//Statements
#[test]
fn task1_3() {
    // Make it work with two ways
        let v = {
            let mut x = 1;
            x += 2
        };

        assert_eq!(v,());

        println!("Success!");
}

#[test]
fn task2_3() {
    let v = {
        let x = 3;
        x
    };
    assert!(v == 3);

    println!("Success!");
}

#[test]
fn task4_3() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

//Functions
#[test]
fn task1_4(){
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum2(x: i32, y: i32) {
    let _  = x + y;
}

#[test]
fn task2_4(){
    print();
}
// Replace i32 with another type
fn print() -> () {
    println!("Success!");
}

// Solve it in two ways
// DON'T let `println!` work
#[test]
fn task3_4() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    panic!("never return");
    // Implement this function, don't modify the fn signatures
}

#[test]
fn task4_4() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
!unimplemented!() // !panic // !todo
}

#[test]
fn task5_4() {
    // FILL in the blank
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}