fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {y}");

    another_function(5);
    print_labeled_measurement(5, 'h');

    let z = five();
    println!("The value of x is {z}");

    let a = plus_one(5);
    println!("The value of a is {a}");
}

fn plus_one(x: u32) -> u32 {
    x + 1
}

fn five() -> u32 {
    5
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The mensurement is {value}{unit_label}");
}
