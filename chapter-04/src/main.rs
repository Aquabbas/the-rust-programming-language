#![allow(unused)]

fn main() {
    let s1 = String::from("Hello!!!");

    let (s2, length) = calculate_length(s1);

    println!("The length of '{s2}' is {length} bytes")
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
