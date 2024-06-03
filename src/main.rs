use std::io;

fn main() {
    println!("GUESS THE NUMBER !!!\n");
    println!("Please input your guess.");

    let mut guessed_number = String::new();

    io::stdin()
        .read_line(&mut guessed_number)
        .expect("Failed to read input");
    println!("You guessed: {guessed_number}");
}
