use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("-------------Guess the number-------------");
    let secret_number = rand::thread_rng().gen_range(1..=5);
    loop {
        println!("Please input your guess between 1 to 5");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed:{}", guess);
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        
        // Error Handling
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Too Big"),
        };
    }
    println!("The secret number was {secret_number}");
}
