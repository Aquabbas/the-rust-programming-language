#![allow(unused)]

fn main() {
    let reference_to_nothing = dangel();
}

fn dangel() -> &String {
    let s = String::from("Helloooo!");

    &s
}
