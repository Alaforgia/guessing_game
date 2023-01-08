use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // mut allows the variable to be mutable. Variables are immutable in Rust by default.
    //
    // " :: " indicates that new is an associated function, which  is implemented on a type, in this
    // case String, rather than on a particular instance of a String.
    // "new" is common on many functions that make a new value of some kind.
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed to read line.");

    println!("You guessed: {}", guess);
}
