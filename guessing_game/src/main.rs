use rand::Rng;
use std::cmp::Ordering; // enum with Less, Greater and Equal
use std::io; // declaring io form std

fn main() { // entry point
    println!("Guess the number!");

    
    let secret_number = rand::thread_rng() // gives the particular random number generator
    .gen_range(1..=100); // takes a range expression as an argument and generates a random number in the range

    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new(); // mut is like let from JS
        
        io::stdin() // stdin comes from io lib
        .read_line(&mut guess) // program waits for input
        // & indicates a reference to guess variable
        .expect("Failed to read line"); // pottential error handling
        
        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            
            
        // this "guess" is a shadow of the original string
    
        println!("You guessed: {guess}"); // prnt variable to terminal
    
        match guess.cmp(&secret_number) { // cmp is used to compare
            // match expression is made up of arms
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}