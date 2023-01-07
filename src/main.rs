use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // mut allows the variable to be mutable. Variables are immutable in RUst by default
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed to read line.");

    println!("You guessed: {}", guess);
}
