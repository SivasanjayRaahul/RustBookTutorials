use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn main() {
    println!("Welcome to the Guessing Game");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Invalid number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("The guess is {}", guess);

        /*
            print! and println! are macros

            let apples = 5; //immutable
            let mut bananas = 8; //mutable
        */

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}