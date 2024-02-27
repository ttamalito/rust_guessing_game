// import the input output library
use std::io;

fn main() {
    // ask the user for input
    println!("Make a guess between 1 and 100!");
    // create a variable to store the user input
    let mut guess = String::new();
    /*
    The let keyword is used to create new variables
    The mut keyword makes the variable mutable
    By default all the variables in Rust are inmmutable
    :: is like a path, in this case is used to access an
    instance or a type, and then we access the member
    of that method through the dot notation.

     */
    // now call the function to accept input
    let amout_bytes = io::stdin().read_line(&mut guess)
                .expect("Failed to read the guess");
    println!("You gave an input of size: {amout_bytes}");
    println!("Which is : {guess}");



    println!("Hello, world!");
}