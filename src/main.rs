use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number! It is between 1 and 100.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guesses = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("Not a humber, go again."); continue; }
        };

        guesses = guesses + 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { println!("You win! Number of guesses: {guesses}"); break; },
        }
    }
}

