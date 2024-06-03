use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("GUESS THE NUMBER !!!\n");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // create a variable to store user guess input
        let mut guessed_number = String::new();

        // read the user input
        io::stdin()
            .read_line(&mut guessed_number)
            .expect("Failed to read input");

        // print user input
        println!("You guessed: {guessed_number}");

        // convert user input into string,
        // in case of invalid conversions,
        // keep asking for numeric string input
        let guessed_number: u32 = match guessed_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // continue asking for input unitl it's a match.
        match guessed_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
