fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    let mut s = String::from("hello");
    change(&mut s);

    println!("The length of '{}' is {}.", s1, len);
    println!("The value of s is {}", s)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
