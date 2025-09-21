use clearscreen; //clearscreen crate
use colored::*; //for output coloring and formatting
use rand::Rng;
use std::io::*; //input output handlíng from standard library
use std::{thread, time}; //thread and time handling from a standard library // Rng trait is needed for gen_range

fn main() {
    clear_console();
    guess();
}

fn guess() {
    // Create a mutable string to store the user's input
    let mut guess = String::new();

    // IMPORTANT: This line has an error. It should be:
    // let number = rand::thread_rng().gen_range(1..=50);
    // Current implementation will not compile - rand::rng() and random_range are not valid methods
    let number = rand::rng().random_range(1..=50);

    loop {
        println!("{}", "Guessing Game!".bold().green());
        println!("Guess a whole number between 1 and 100:");

        // Read user input and handle potential errors
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line from stdin");

        match guess.trim().parse::<i32>() {
            Ok(guess_int) => {
                if guess_int == number {
                    println!("You guessed it! the number was {}", number);
                    break;
                } else if guess_int > number {
                    println!("Too high!");
                    thread::sleep(time::Duration::from_millis(500));
                    clear_console();
                    guess.clear();
                } else if guess_int < number {
                    println!("Too low!");
                    thread::sleep(time::Duration::from_millis(500));
                    clear_console();
                    guess.clear();
                }
            }

            Err(_) => {
                println!("A whole number buddy");
                thread::sleep(time::Duration::from_millis(500));
                clear_console();
                guess.clear();
                continue;
            }
        }
    }
}

fn clear_console() {
    // Use the clearscreen crate to clear the terminal
    clearscreen::clear().expect("Failed to clear the console screen");
}
