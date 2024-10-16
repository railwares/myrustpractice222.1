// Fill the blanks
enum Direction {
    East,
    West,
    North,
    South,
}
#[test]
fn task1_1() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => { // Matching South or North here
            println!("South or North");
        },
        Direction::West => println!("West"),
    };
}

#[test]
fn task2_1() {
    let boolean = true;

    // Fill the blank with a match expression:
    //
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary = match boolean {
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);

    println!("Success!");
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[test]
fn task3_1() {
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }

    println!("Success!");
}

fn show_message(msg: Message) {
    match msg {
        Message::Move {x:a, y: b} => { // match  Message::Move
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        }
        _ => println!("no data in these variants")
    }
}

#[test]
fn task4_1() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    // Fill the blank with `matches!` to make the code work
    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }

    println!("Success!");
}

enum MyEnum {
    Foo,
    Bar
}


#[test]
fn task5_1() {
    let mut count = 0;

    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        if matches!(e, MyEnum::Foo) { // Fix the error by changing only this line
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success!");
}

//
#[test]
fn task6_1() {
    let o = Some(7);

    // Remove the whole `match` block, using `if let` instead
         if let Some(i) = o {
            println!("This is a really long string and `{:?}`", i);

            println!("Success!");
        }
}


// Fill in the blank
enum Foo {
    Bar(u8)
}

#[test]
fn task7_1() {
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
}


enum Foo3 {
    Bar,
    Baz,
    Qux(u32)
}

#[test]
fn task8_1() {
    let a = Foo3::Qux(10);

    // Remove the codes below, using `match` instead
    match a {
        Foo3::Bar => println!("match foo::bar"),
        Foo3::Baz => println!("match foo::baz"),
        _ => println!("match others")
    }
}

#[test]
// Fix the errors in-place
fn task9_1() {
    let age = Some(30);
    if let Some(age) = age { // Create a new variable with the same name as previous `age`
        assert_eq!(age, 30);
    } // The new variable `age` goes out of scope here

    match age {
        // Match can also introduce a new shadowed variable
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
}


//8.2patterns
#[test]
fn task1_2() {}
fn match_number(n: i32) {   // Use | to match several values, use ..= to match an inclusive range.
    match n {
        // match a single value
        1 => println!("One!"),
        // fill in the blank with `|`, DON'T use `..` ofr `..=`
        2 | 3 | 4 | 5 => println!("match 2 -> 5"),
        // match an inclusive range
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => {
            println!("match 11 -> +infinite")
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}
#[test]
fn task2_2() {   //The @ operator lets us create a variable that holds a value, at the same time we are testing that value to see whether it matches a pattern.
    // Fill in the blank to let p match the second arm
    let p = Point { x: 2, y: 20 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // Second arm
        Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}


// Fix the errors
enum Message2 {
    Hello { id: i32 },
}
#[test]
fn task3_2() {
    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello {
            id:  id@3..=7,
        } => println!("Found an id in range [3, 7]: {}", id),
        Message2::Hello { id: newid@(10 | 11 | 12) } => {
            println!("Found an id in another range [10, 12]: {}", newid)
        }
        Message2::Hello { id } => println!("Found some other id: {}", id),
    }
}

// Fill in the blank to make the code work, `split` MUST be used
#[test]
fn task4_2() {  //A match guard is an additional if condition specified after the pattern in a match arm that must also match,
    // along with the pattern matching, for that arm to be chosen.
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}

#[test]
fn task5_2() { // Ignoring remaining parts of the value with ..
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first,..,last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }

    println!("Success!");
}

#[test]
fn task6_2() { // FIX the error with least changing  //Using pattern &mut V to match a mutable reference requires you to be very careful,
    // due to V being a value after matching.
    // DON'T remove any code line
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        value => value.push_str(" world!")
    }

}