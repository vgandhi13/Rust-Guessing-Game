use std::io;
use std::cmp::Ordering;    
use rand::Rng; //This line imports the Rng trait from the rand crate. The Rng trait provides methods for generating random numbers.

fn main() {
    println!("Enter a number: ");
    let secret_number = rand::thread_rng().gen_range(1..=100); //rand::thread_rng() creates a random number generator specific to the current thread. This function sets up a random number generator that is local to the current thread, ensuring that it is properly seeded for generating random numbers.
    // println!("Secret number is {secret_number}");

    loop {
        println!("Please Enter your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {  // parse returns a Result type and Result is an enum that has the variants Ok and Err
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}