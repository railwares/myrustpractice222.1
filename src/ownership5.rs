//5.1Ownership

#[test]
fn task1_1(){
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    let y = x.clone();
    println!("{}, {}",x, y);
}

#[test]
fn task2_1(){
    // Don't modify code in main
        let s1 = String::from("Hello world");
        let s2 = take_ownership(s1);

        println!("{}", s2);
    }

    // Only modify the code below!
    fn take_ownership(s: String) -> String {
        println!("{}", s);
        s
}

#[test]
fn task3_1(){
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.as_bytes();
    s
}

#[test]
fn task4_1(){
    let s = String::from("Hello World");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}


#[test]
fn task5_1(){
    let x = (1, 2, (), "hello");  // Don't use clone ,use copy instead
    let y = x;
    println!("{:?}, {:?}", x, y);
}

#[test]
fn task6_1(){
    let s = String::from("Hello "); // make the necessary variable mutable

    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}

#[test]
fn task7_1(){
    let x = Box::new(5);

    let mut y = Box::new(3);    // update this line, don't change other lines!

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}

#[test]
fn task8_1(){
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0;

    // Modify this line only, don't use `_s`
    println!("{:?}", t.1);
}


#[test]
fn task9_1(){
    let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}


//5.2Borrowing

#[test]
fn task1_2(){
    let x = 5;
    // Fill the blank
    let p = &x;

    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}

#[test]
fn task2_2(){
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(&x, y);

    println!("Success!");
}

#[test]
fn task3_2(){
    let mut s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {}


#[test]
fn task4_2(){
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

#[test]
fn task5_2(){
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s;

    p.push_str("world");

    println!("Success!");
}

#[test]   //ref can be used to take references to a value, similar to &
fn task6_2(){
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

#[test]
fn task7_2(){
    // Remove something to make it work
    // Don't remove a whole line !
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}

#[test]
fn task8_2(){
    // Fix error by modifying this line
    let mut s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object2(s: &mut String) {}


#[test]
fn task9_2(){ //Ok: Borrow a mutable object as immutable // This code has no errors!
    let mut s = String::from("hello, ");

    borrow_object(&s);

    s.push_str("world");

    println!("Success!");
}

fn borrow_object3(s: &String) {}


#[test]
fn task10_2(){
    let mut s = String::from("hello, ");
    // Comment one line to make it work
    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");

    //println!("{}",r1);
}

#[test]
fn task11_2(){
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;
    //r1.push_str("world");
    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
}