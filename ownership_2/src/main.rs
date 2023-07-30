fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    let mut s = String::from("hello");
    change(&mut s);

    println!("The length of '{}' is {}.", s1, len);
    println!("The value of s is {}", s);

    let s2 = String::from("hello world");
    let word = first_word(&s2);

    let hello = &s2[..5];
    let world = &s2[6..11];

    println!("hello, world {} {}", hello, world);
    println!("word is {}", word);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
