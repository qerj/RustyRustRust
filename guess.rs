use std::{io, ptr::read_unaligned, cmp::Ordering};
use rand::Rng;


fn main() {
    println!("Guess the number: ");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number: {secret_number}");

    println!("Please guess the secret number: ");


    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You gussed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("YOU GUESSED THE NUMBER!")
    }
}
