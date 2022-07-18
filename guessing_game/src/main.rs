use std::io; // declaring io form std

fn main() { // entry point
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // mut is like let from JS

    io::stdin() // stdin comes from io lib
        .read_line(&mut guess) // program waits for input
        // & indicates a reference to guess variable
        .expect("Failed to read line"); // pottential error handling

    println!("You guessed: {guess}"); // prnt variable to terminal
}