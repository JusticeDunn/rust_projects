use std::io;

fn main() {
    println!("Guess the number I'm thinking!");

    println!("Input your guess now.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
