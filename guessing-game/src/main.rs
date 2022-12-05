use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

fn main() {
    println!("Lets guess a number from 1-100!");

    println!("Please input your guess.");

    // let mut -> mutable variable
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        // & -> pass by reference
        .expect("Failed to read line");
    
        print!("You guessed: {guess}");

    let guess: u32 = guess.trim().parse().expect("Please only enter numbers");

    let mut rng = thread_rng();
    let answer = rng.gen_range(0..100);
    
    // Exhaustive handling of conditional statements
    match guess.cmp(&answer) {
        Ordering::Less => print!("Too low, nice try"),
        Ordering::Greater => print!("Too high, nice try"),
        Ordering::Equal => print!("** CORRECT **"),
    }
}
