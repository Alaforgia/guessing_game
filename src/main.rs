use std::io;

use std::cmp::Ordering;

// The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods. 
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // "rand::thread_rng" function will give us the particular random number generator that we’re going to use: one that is local 
    // to the current thread of execution and seeded by the operating system.
    // "gen_range" method is defined by the Rng trait that we brought into scope with the use rand::Rng statement. 
    // The gen_range method takes two numbers as arguments and generates a random number between them. It’s inclusive on the 
    // lower bound but exclusive on the upper bound, so we need to specify 1 and 101 to request a number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(0..101);

    println!("The secret number is: {}", secret_number);

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
     // "read_line" returns a value (in this case " io::Result ") on top of putting whatever is passed into it, into a String.
     // Result types are enumerations, often referred to as enums. An enumeration is a type that can have a fixed set of values,
     // and those values are called the enum’s variants. 
    io::stdin().read_line(&mut guess).expect("failed to read line.");


    // " {} " is a placeholder of a value. Below, our "guess" will appear where the curly brackets are.
    // So if you cargo run, the terminal will prompt you and it will look like this:
    // Guess the number!
    // Please input your guess.
    // 6
    // You guessed: 6
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
