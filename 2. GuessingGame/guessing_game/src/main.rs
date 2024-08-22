use rand::Rng;
use std::cmp::Ordering;
use std::io; // Use input/output library.

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new(); // Create a mutable variable binds to a new, empty string.
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess) // & indicates a reference and immutable by default, so we need to add mut to make it mutable
            .expect("Failed to read line."); // Handle the Result type, if Result is a Err value, expect() will cause the program to crash and display error message

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, try again.");
                continue;
            }
        }; // Convert guess to integer to be able to compare with secret_number

        println!("Your guess is {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Greater => println!("Too large!"),
            Ordering::Less => println!("Too small!"),
        }
    }
}
