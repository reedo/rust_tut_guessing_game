use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// The entry point to the program.
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");

        // creates a new, mutable string
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // Passes in guess as a mutable reference, so the original is modified by the function, not a copy.
            .expect("Failed to read line"); // `read_line` returns a `Result`, which if an `Err` will exit the program with the passed in error message.

        /*
         * Convert guess to a `u32`.
         * Rust allows us to shadow the previous value of guess with a new one.
         * This feature is often used in situations in which you want to convert a value from one type to another type.
         */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        /*
         * Strings can have `{}` as text placeholders. the values passed in after the string are the replacements.
         * They will be substituted in the same order they are passed in.
         * So a string with three instances of `{}` will need 3 values passed in following the string.
         */
        println!("You guessed: {}", guess);

        /*
         * Compare the guess with the secret number.
         */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
