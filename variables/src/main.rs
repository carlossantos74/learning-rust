// constant
// const THRREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // immutable variable
    let y = 5;

    // Shadowing
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }

    println!("The value of y is: {y}");

    // mutable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
