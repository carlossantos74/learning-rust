use std::io;

// Integer Types

/*
Unsigned integers (u8, u16, u32, u64, u128) can only represent non-negative values, while signed integers (i8, i16, i32, i64, i128) can represent both positive and negative values.
For example, an u8 can represent values from 0 to 255, while an i8 can represent values from -128 to 127.
When choosing between signed and unsigned integers, it's important to consider the range of values that you need to represent, as well as whether negative values are relevant to your use case.
*/

// Integer Overflow

/*
Integer overflow occurs when the result of an arithmetic operation on an integer type is too large or too small to be represented by the type.
For example, if you add 1 to the maximum value of an u8 (255), the result will be 256, which is too large to be represented by an u8. In this case, the value will "wrap around" to 0, resulting in an incorrect result.
In Rust, integer overflow is considered a programming error, and Rust's default behavior is to panic at runtime when an overflow occurs. This behavior can be changed by using the Wrapping type, which allows arithmetic operations to wrap around instead of panicking.
It's important to be aware of the possibility of integer overflow when working with integer types, and to choose the appropriate type for the range of values that you need to represent.
*/

// Type  | Signed | Unsigned | Size (bytes)
// -----------------------------------------
// i8    |   ✓    |          |      1
// i16   |   ✓    |          |      2
// i32   |   ✓    |          |      4
// i64   |   ✓    |          |      8
// i128  |   ✓    |          |      16
// u8    |        |    ✓     |      1
// u16   |        |    ✓     |      2
// u32   |        |    ✓     |      4
// u64   |        |    ✓     |      8
// u128  |        |    ✓     |      16

// Floating-Point Types
// Type  | Size (bytes) | Precision
// ---------------------------------
// f32   |      4      |  6-9 digits
// f64   |      8      | 15-17 digits

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}

fn boolean_type() {
    // implicit type annotation
    let t = true;

    // explicit type annotation
    let f: bool = false;
}

fn character_type() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

fn tuple_type() {
    let tup: (u32, f64, char) = (500, 6.4, 'z');

    // partern matching
    let (x, y, z) = tup;

    // accessing tuple elements
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let zee = tup.2;
}

fn array_type() {
    // implicit type annotation => [char; 12]
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // explicit type annotation
    let a: [u32; 5] = [1, 2, 3, 4, 5];

    // initializing array with same value
    let b = [3; 5]; // => [3, 3, 3, 3, 3]

    // accessing array elements
    let first = a[0];
    let second = a[1];
}

fn invalid_array_access() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Intex entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is {element}");
}

fn main() {
    // println!("Hello, world!");
    invalid_array_access();
}
