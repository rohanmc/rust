extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    // println! is a macro.
    println!("Guess the number!");
    // random num gen local to this thread
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input you guess.");

    // let is used to assign variables
    // default all variables and references are immutable, hence use mut / &mut to make them mutable
    let mut guess = String::new();

    // rust will warn you for unused Result object's exception handling
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    // anything in curly braces is replaced with variables following it
    println!("You guess: {}", guess);
}
