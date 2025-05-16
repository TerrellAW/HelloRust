use std::io;
use rand::prelude::*;

fn main() {
    println!("Hello, Rust!");

    println!("Let's guess the number!");

    // Initialize guess
    let mut guess = String::new();

    // Take user input for guess
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Parse guess to remove extra spaces
    let guess = guess.trim();

    // Initialize and parse randomly chosen number
    let mut rng = rand::rng(); // Randomize
    let nums: Vec<i32> = (1..100).collect(); // Enumerate int from 1 to 100
    let num = nums.choose(&mut rng); // Choose a number randomly from enum
    let num_str = match num {
        Some(n) => n.to_string(), // Change int to string
        None => String::from("None"), // Handle void
    };

    println!("You guessed: {}, Machine guessed: {}", guess, num_str);

    let is_match = guess.contains(&num_str);

    if is_match {
        println!("You guessed correctly!");
    }
    else {
        println!("Incorrect guess.");
    }
}
