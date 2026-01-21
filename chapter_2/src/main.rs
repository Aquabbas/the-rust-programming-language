use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("Guess the number, my dude.");

    let secret = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {secret}");

    loop {
        let mut number = String::new();
        stdin().read_line(&mut number).expect("Awwww Shiiiiiit");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {number}");

        match number.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
