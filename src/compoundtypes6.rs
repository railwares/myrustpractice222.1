//6.1string

#[test]   //The type of string literal "hello, world" is &str, e.g let s: &str = "hello, world".
fn task1_1(){  //We can't use str type in normal ways, but we can use &str. // Fix error without adding new line
    let _s: &str = "hello, world";

    println!("Success!");
}

#[test]
fn task2_1(){
    let s: Box<&str> = "hello, world".into();  //We can only use str by boxing it, & can be used to convert Box<str> to &str // Fix the error with at least two solutions
    greetings(*s)  //Box<&str>, (*s) - alt solution
}

fn greetings(s: &str) {
    println!("{}",s)
}

#[test] //String type is defined in std and stored as a vector of bytes (Vec), but guaranteed to always be a valid UTF-8 sequence.
// String is heap allocated, growable and not null terminated.
fn task3_1(){
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

#[test]  // Fix all errors without adding newline
fn task4_1(){
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);
}

#[test]
fn task5_1(){  //replace can be used to replace substring
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

#[test]  // Fix errors without removing any line //You can only concat a String with &str, and String's ownership can be moved to another variable.
fn task6_1(){
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}

#[test] //Opposite to the seldom using of str, &str and String are used everywhere!
fn task7_1(){  //&str can be converted to String in two ways
    let s = "hello, world".to_string();
    greetings2(s);
}

fn greetings2(s: String) {
    println!("{}", s)
}

#[test]
// Use two approaches to fix the error and without adding a new line
fn task8_1(){
    let s = "hello, world".to_string();  //We can use String::from or to_string to convert a &str to String
    let _s1 = s; //alt: s1: &str = &s

    println!("Success!");
}

#[test]
fn task9_1(){
    // You can use escapes to write bytes by their hexadecimal values
    // Fill the blank below to show "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73\x74!"; // \x73,\74 = s,t
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...Or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
             unicode_codepoint, character_name );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

#[test]
fn task10_1(){
    // Sometimes there are just too many characters that need to be escaped or it's just much more convenient to write a string out as-is.
    // This is where raw string literals come into play.
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    // Modify above line to make it work
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Fill the blank
    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
}

#[test] //You can't use index to access a char in a string, but you can use slice &s1[start..end].
fn task11_1(){
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..6]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");

    println!("Success!");
}

#[test]
fn task12_1(){
    // // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}

//6.2Array
//The type of array is [T; Length], as you can see, array's length is part of their type signature.
// So their length must be known at compile time.

#[test]
fn task1_2(){
    // Fill the blank with proper array type
    let arr = [1, 2, 3, 4, 5];

    // Modify the code below to make it work
    assert!(arr.len() == 5);

    println!("Success!");
}

#[test]
fn task2_2(){
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let _arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];

    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12);

    println!("Success!");
}

#[test]
fn task3_2(){
    // Fill the blank
    let list: [i32; 100] = [1; 100];

    assert!(list[0] == 1);   //All elements in an array can be initialized to the same value at once.
    assert!(list.len() == 100);

    println!("Success!");
}

#[test]
fn task4_2(){
    // Fix the error
    let _arr = [1, 2, 3];  //All elements in an array must be of the same type

    println!("Success!");
}

#[test]
fn task5_2(){
    let arr = ['a', 'b', 'c'];

    let ele = arr[0]; // Indexing starts at 0. // Only modify this line to make the code work!

    assert!(ele == 'a');

    println!("Success!");
}

#[test]
fn task6_2(){
    let names = [String::from("Sunfei"), "Sunface".to_string()];

    // `Get` returns an Option<T>, it's safe to use
    let _name0 = names.get(0).unwrap();

    // But indexing is not safe  //Out of bounds indexing causes panic
    let _name1 = &names[0];

    println!("Success!");
}

//6.3Slice

#[test]  //Here, both [i32] and str are slice types, but directly using it will cause errors.
// You have to use the reference of the slice instead: &[i32], &str.
fn task1_3(){
    let arr = [1, 2, 3];
    let _s1: &[i32] = &arr[0..2];

    let _s2 = "hello, world";

    println!("Success!");
}

#[test]
fn task2_3(){
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success!");
}

#[test]
fn task3_3(){
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}

#[test]
fn task4_3(){
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);

    println!("Success!");
}

#[test]
fn task5_3(){
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..3]; // it is inside '你' (bytes 0..3) of `你好，世界`

    assert!(slice == "你");

    println!("Success!");
}

#[test]
fn task6_3(){
    let mut s = String::from("hello world");

    // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
    // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`.
    let letter = first_letter(&s);
    println!("the first letter is: {}", letter);
    s.clear(); // error!

}
fn first_letter(s: &str) -> &str {
    &s[..1]
}

//6.4Tuple
//Elements in a tuple can have different types. Tuple's type signature is (T1, T2, ...),
// where T1, T2 are the types of tuple's members.
#[test]
fn task1_4(){
    let _t0: (u8,i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // Fill the blanks to make the code work
    let _t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}

#[test]
fn task2_4(){
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");
    println!("Success!");
}

#[test]
fn task3_4(){
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
}

#[test]
fn task4_4(){
    let tup = (1, 6.4, "hello");

    // Fill the blank to make the code work
    let (x, y, z) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, 6.4);
    assert_eq!(z, "hello");

    println!("Success!");
}

#[test]
fn task5_4(){
    let (x, y, z);

    // Fill the blank
    (y, z, x) = (1, 2, 3);

    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success!");
}

#[test]
fn task6_4(){
    // Fill the blank, need a few computations here.
    let (x, y) = sum_multiply((2, 3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

//6.5Struct


    // Fix the error
    struct Person {
        name: String,
        age: u8,
        hobby: String
    }
#[test]
    fn task1_5() {
        let age = 30;
        let _p = Person {
            name: String::from("sunface"),
            age,
            hobby: String::from("hobby")
        };

        println!("Success!");
}


struct Unit;
trait SomeTrait {
    // ...Some behaviors defined here.
}

// We don't care about what fields  are  in the Unit, but we care about its behaviors.
// So we use a struct with no fields and implement some behaviors for it
impl SomeTrait for Unit {  }
#[test]
fn task2_5() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
}

// Fill the blank to make the code work
fn do_something_with_unit(_u: Unit) {   }


// Fix the error and fill the blanks
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
#[test]
fn task3_5() {
    let v = Point(0, 127, 255);
    check_color(v);

    println!("Success!");
}

fn check_color(p: Point) {
    let Point(x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
}


// Fill the blank and fix the error without adding/removing new line
struct Person2 {
    name: String,
    age: u8,
}
#[test]
fn task4_5() {
    let age = 18;
    let mut p2 = Person2 {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18?
    p2.age = 30;

    // Fill the blank
    p2.name = String::from("sunfei");

    println!("Success!");
}


// Fill the blank
struct Person3 {
    name: String,
    age: u8,
}
#[test]
fn task5_5() {
    println!("Success!");
}

fn build_person(name: String, age: u8) -> Person3 {
    Person3 {
        age,
        name
    }
}

//6.6Enum

    enum Number {
        Zero,
        One,
        Two,
    }

    enum Number1 {
        Zero = 0,
        One,
        Two,
    }

    // C-like enum
    enum Number2 {
        Zero = 0,
        One = 1,
        Two = 2,
    }

#[test]
    fn task1_6() {
        // An enum variant can be converted to a integer by `as`
        assert_eq!(Number::One as u8, Number1::One as u8);
        assert_eq!(Number1::One as u8, Number2::One as u8);

        println!("Success!");
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[test]
fn task2_6() {
    let _msg1 = Message3::Move{x: 1, y: 2}; // Instantiating with x = 1, y = 2
    let _msg2 = Message3::Write (String::from("hello, world")); // Instantiating with "hello, world!"

    println!("Success!");
}

enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[test]
fn task3_6() {
    let msg = Message2::Move{x: 1, y: 1};

    if let Message2::Move{x: a, y: b} = msg {
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN！");
    }

    println!("Success!");
}

#[derive(Debug)]
enum Message3 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[test]
fn task4_6() {  // Fill in the blank and fix the errors
    let msgs: [Message3; 3] = [
        Message3::Quit,
        Message3::Move{x:1, y:3},
        Message3::ChangeColor(255, 255, 0)
    ];

    for msg in msgs {
        show_message(msg)
    }
}

fn show_message(msg: Message3) {
    println!("{:?}", msg);
}

//Since there is no null in Rust, we have to use enum Option<T> to deal with the cases when the value is absent.
// Fill in the blank to make the `println` work.
// Also add some code to prevent the `panic` from running.
#[test]
fn task5_6() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(n) = six {
        println!("{}", n);

        println!("Success!");
        return;
    }

    panic!("NEVER LET THIS RUN！");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}