use std::io;

fn main() {
    println!("Enter a number: ");

    println!("Please Enter your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {guess}");
}