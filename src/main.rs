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

    // "read_line" takes whatever the user types into standard input and place that into a string, so it takes that string as an argument.
     // "&" indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data 
     // without needing to copy that data into memory multiple times.
     // References, like variables are immutable by default, so we need to write "&mut guess".
    io::stdin().read_line(&mut guess).expect("failed to read line.");

    println!("You guessed: {}", guess);
}
