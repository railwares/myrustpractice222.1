#[test]
fn task1() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}
// Use mut to mark a variable as mutable.
#[test]
fn task2() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}
// A scope is the range within the program for which the item is valid.
#[test]
fn task3() {
    let x: i32 = 10;
    let y: i32 = 20;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}

#[test]
fn task4() {
    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> String {
    let x = "hello".to_string();
    x

}

// Shadowing. You can declare a new variable with the same name as a previous variable, here we can say the first one is shadowed by the second one.

#[test]
fn task5() {  // Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}


#[test]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn task6() {  // Remove a line in the code to make it compile
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!";

    println!("Success!");
}
// Unused variables
#[test]
fn task7() {
    let _x = 1;
}

#[test]
fn task8() {  // Tips: you can use Shadowing or Mutability
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

#[test]
fn task9() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3, 2]);
    println!("Success!");
}