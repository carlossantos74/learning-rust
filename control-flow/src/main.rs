fn main() {
    let number = 3;

    // check if number is less than 5
    if number < 5 {
        println!("condition was true")
    } else {
        println!("condition was false")
    }

    // INVALID - rust dont automatically convert non-boolean types to boolean
    // if number {
    //   println!("number was three");
    // }

    // handle multiple conditions
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // using if in a let statement
    let condition = true;

    let x = if condition { 5 } else { 6 };

    println!("The value of x is: {x}");

    // loop
    loop {
        println!("again!");
        break;
    }

    // returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // loop labels
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaing = 10;

        loop {
            println!("remaing = {remaing}");
            if remaing == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaing -= 1;
        }
        count += 1;
    }

    println!("End count = {count}");

    // conditional loops with while

    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // looping through a collection with while and for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value in while is: {}", a[index]);

        index += 1;
    }

    // using a for loop and a range
    for element in a {
        println!("the value in for is: {}", element);
    }

    // using a for loop and a range
    for number in (1..4).rev() {
        println!("{number}");
    }
}
