#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

struct A;          // Concrete type `A`.
struct S(A);       // Concrete type `S`.
struct SGen<A>(A); // Generic type `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<A>(_s: SGen<A>) {}
#[test]
fn task1_1() {
    // Using the non-generic functions
    reg_fn(S(A));          // Concrete type.
    gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(6)); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('a'));

    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen('c'));

    println!("Success!");
}


//A function call with explicitly specified type parameters looks like: fun::<A, B, ...>().
fn sum<A:std::ops::Add<Output = A>>(x: A, y: A) -> A {
    x + y
}
#[test]
fn task2_1() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    println!("Success!");
}


struct Point<A> {
    x: A,
    y: A,
}
// Implement struct Point to make it work.
#[test]
fn task3_1() {
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };

    println!("Success!");
}


// Modify this struct to make the code work
struct Point2<A, B> {
    x: A,
    y: B,
}
#[test]
fn task4_1() {
    // DON'T modify this code.
    let _p = Point2{x: 5, y : "hello".to_string()};

    println!("Success!");
}

// Add generic for Val to make the code work, DON'T modify the code in `main`.
struct Val<A> {
    val: A,
}

impl<A> Val<A> {
    fn value(&self) -> &A {
        &self.val
    }
}
#[test]
fn task5_1() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}


struct Point3<A, B> {
    x: A,
    y: B,
}

impl<A, B> Point3<A, B> {
    fn mixup<C, D>(self, other: Point3<C, D>) -> Point3<A, D> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}
#[test]
fn task6_1() {
    let p1 = Point3 { x: 5, y: 10 };
    let p2 = Point3 { x: "Hello", y: '中'};

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    println!("Success!");
}


// Fix the errors to make the code work.
struct Point4<T> {
    x: T,
    y: T,
}

impl Point4<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
#[test]
fn task7_1() {
    let p = Point4{x: 5.0_f32, y: 10.0_f32};
    println!("{}",p.distance_from_origin());
}

//2. Const generics

//<T, const N: usize> is part of the struct type, it means Array<i32, 3> and Array<i32, 4> are different types.
struct Array<T, const N: usize> {
    data : [T; N]
}
#[test]
fn task1_2() {
    let _arrays = [
        Array{
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 4]
        }
    ];

    println!("Success!");
}


// Fill in the blanks to make it work.
fn print_array<A: std::fmt::Debug, const N: usize>(arr: [A; N]) {
    println!("{:?}", arr);
}
#[test]
fn task2_2() {
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}



//#[allow(incomplete_features)]
//#[feature(generic_const_exprs)]

//fn check_size<T>(_val: T)
//where
//   Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue, //cannot perform const operation using `T`????
//{
//    //...
//}

// fix the errors in main
//#[test]
//fn main() {
    //check_size([0u8; 767]);
    //check_size([0i32; 191]);
    //check_size(["hello你好"; 47]); // &str is a string reference, containing a pointer and string length in it, so it takes two word long, in x86-64, 1 word = 8 bytes
    //check_size([(); 31].map(|_| "hello你好".to_string()));  // String is a smart pointer struct, it has three fields: pointer, length and capacity, each takes 8 bytes
    //check_size(['中'; 191]); // A char takes 4 bytes in Rust

//   println!("Success!");
//}

//pub enum Assert<const CHECK: bool> {}

//pub trait IsTrue {}

//impl IsTrue for Assert<true> {}



//3. Traits
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
fn task1_3() {
    let s = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");

    println!("Success!");
}



// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// Add some attributes to make the code work
// DON'T modify other codes
#[derive(Debug,PartialEq,PartialOrd)]
struct Seconds(i32);
#[test]
fn task2_3() {
    let _one_second = Seconds(1);

    println!("One second looks like: {:?}", _one_second);
    let _this_is_true = (_one_second == _one_second);
    let _this_is_false = (_one_second > _one_second);

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}



