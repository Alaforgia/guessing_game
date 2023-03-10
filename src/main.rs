use std::io;

// Like Result, Ordering is another enum, but the variants for Ordering are Less, Greater, and Equal.
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

    // println!("The secret number is: {}", secret_number);

    loop {
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
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line.");

        // Rust allows us to shadow the previous value of guess with a new one. This feature is often used in
        // situations in which you want to convert a value from one type to another type.
        // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess for example.
        //
        //
        // The guess below refers to the original guess that was a String.
        // The trim method on a String will eliminate any whitespace at the beginning and end.
        //
        //The parse method on strings parses a string into some kind of number. Because this method can parse a variety of number types,
        // we need to tell Rust the exact number type we want by using let guess: u32. The colon (:) after guess tells Rust
        // we’ll annotate the variable’s type.
        //
        // The u32 seen here is an unsigned, 32-bit integer. It’s a good default choice for a small positive number.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // "_" is a catchall value, we want to match all error values no matter what info they have.
            // continue will tell the program to to go to the next iteration of the loop and ask for another guess.
            // The program will ignore all errors that parse might encounter.
            Err(_) => continue,
        };

        // " {} " is a placeholder of a value. Below, our "guess" will appear where the curly brackets are.
        // So if you cargo run, the terminal will prompt you and it will look like this:
        // Guess the number!
        // Please input your guess.
        // 6
        // You guessed: 6
        println!("You guessed: {}", guess);

        // The cmp method w compares two values and can be called on anything that can be compared.
        // It takes a reference to whatever you want to compare with: here it’s comparing the guess to the secret_number.
        // Then it returns a variant of the Ordering enum we brought into scope with the use statement.
        //
        //
        // A match expression is made up of arms. An arm consists of a pattern and the code that should be run
        // if the value given to the beginning of the match expression fits that arm’s pattern. Rust takes the value given to
        // match and looks through each arm’s pattern in turn. The match construct and patterns are powerful features in Rust that
        // let you express a variety of situations your code might encounter and make sure that you handle them all.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
            // break makes the program exit the loop when the user guesses the secret number correctly.
                break;
            }
        }
    }
}
