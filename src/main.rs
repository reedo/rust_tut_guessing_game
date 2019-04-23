use std::io;

// the entry point to the program
fn main() {
	println!("Guess the number!");

	println!("Please input your guess.");

	// creates a new, mutable string
	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess) // passes in guess as a mutable reference, so the original is modified by the function, not a copy
		.expect("Failed to read line"); // read_line resturns a Result, which if an Err will exit the program with the passed in error message

	/*
	 * strings can have {} as text placeholders. the values passed in after the string are the replacements
	 * they will be substituted in the same order they are passed in
	 * so a string with three instances of {} will need 3 values passed in following the string
	 */
	println!("You guessed: {}", guess);
}
