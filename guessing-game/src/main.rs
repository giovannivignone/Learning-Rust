use std::io;

fn main() {
    println!("Lets guess a number!");

    println!("Please input your guess.");

    // let mut -> mutable variable
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        // & -> pass by reference
        .expect("Failed to read line");
    
        print!("You guessed: {guess}");

}
