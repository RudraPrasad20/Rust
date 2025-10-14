// std - use standard library
// io - input output
use std::io;
// to generate a random number every single time, it comes after changing .toml file - dependencys
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number between 1 - 100!");
    // generate a secreat number between 1 to 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please Enter your guess.");
        //  string type, new function, creates a new empty string
        let mut guess = String::new();
        // .read_line - accept what user enters & store that as a string
        // variable & referances are immutable by default
        // &mut guess - mutable
        // &guess - immutable
        // .expect is for erro, if some error happens in .read_line it will return the expected value
        // If you don’t call expect, the program will compile, but you’ll get a warning:
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // remeber while we were storing a new number, we were storing that in a string
        // here we are changing to u32
        // if input is num => return num
        // if input is something else(string etc..) => let them guess again
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number.");
                continue;
            }
        };

        println!("You guessed: {}", guess);
        // cmp was imported earlier, at the top
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
