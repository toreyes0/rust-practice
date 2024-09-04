// std::io::stdin
// library::module::function
use std::io;
use std::cmp::Ordering; // type
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // type::associated function
        let mut guess = String::new();

        // function returns a Result (enum), one of multiple possible states
        // "variant" - possible state
        // readline variants - Ok & Err
        io::stdin()
            // & - reference
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // placeholder
        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) { // expression
            Ordering::Less => println!("Too small"), // arm
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
