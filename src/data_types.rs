use std::io;

pub fn data_types_floating() {
    let x = 2.0; // double
    let y: f32 = 3.0; // float
}

pub fn data_types_numeric_operations() {
    let sum = 5 + 10;  // addition

    let difference = 95.5 - 4.3; // subtraction

    let product = 4 * 30; // multiplication

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    let remainder = 43 % 5; // remainder
}

pub fn data_types_bool() {
    let b = true;
    let b: bool = false; // with explicit type annotation
}

pub fn data_types_char() {
    let c = 't';
    let z: char = 'T'; // with explicit type annotation
    let penguin = '🐧';
}

/* Compound types */
pub fn data_types_tuple() {
    let tup: (i32, bool, &str) = (20, true, "Hello, World!");

    let tup = (200, 6.9, 28);
    let (x, y, z) = tup;
    print!("The value of z is: {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The value of six_point_four is: {six_point_four}");
}

pub fn data_types_array() {
    let a = [28, 187, 69, 50, 2];

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    // Creates an array with data type int with size of 5
    let a: [i32; 5] = [21, 24, 35, 42, 589];
    // Array that contains 5 elements with the value 3
    let a = [3; 5];
}

pub fn data_types_array_accessing() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}

// Throws a runtime error if the index is invalid!
pub fn data_types_array_invalid_access() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin().
        read_line(&mut index).
        expect("Failed to read index!");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number!");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}