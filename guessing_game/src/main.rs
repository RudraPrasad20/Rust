// std - use standard library
// io - input output
use std:: io;
// to generate a random number every single time, it comes after changing .toml file - dependencys
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

	// mut - mutate
	// of string type, new function, creates a new empty string
    let mut guess = String::new();

	//stdin - take input from user
    io::stdin()
    // .read_line - accept what user enters & store that as a string
    // variable & referances are immutable by default
    // &mut guess - mutable
    // &guess - immutable
        .read_line(&mut guess)
    // .expect is for erro, if some error happens in .read_line it will return the expected value
    // If you don’t call expect, the program will compile, but you’ll get a warning:
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}