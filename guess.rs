use core::num;
use std::{io, ptr::read_unaligned, cmp::Ordering};
use rand::Rng;
//^^^ these here are the same as using "using namespace std" in c++. so we dont have to type that much.

fn main() {
    println!("Guess the number: ");

    //secret number is a unsigned 32bit integer that will range from (begin 1) to (end 100)
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    //using the braces {} we can print variables inside them.
    println!("The secret number: {secret_number}");

    //our loop so incase the user anwsers something other than the correct anwser or incorrect type. We go and restart everything in this loop.
    loop { 
    println!("Please guess the secret number: ");

    //type guess is equal to the type string just like "std::string" in c++. With the keyword mut(mutable) meaning its subjected to change.
    let mut guess = String::new();

    //using input/output we can read the line with the reference of guess that will be read into the buffer with the expect call if the code fails.
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    //since we read a string we simply must parse our type into a int for it to be compared to the secret rng number. And it must match the secret_number type that being "u32"(usigned 32 bit integer)
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,  //the "match" will yield a "Ok", and "Err" which we will use. If the parse goes through correctly "Ok" will be set to the number".
        Err(_) => continue,//if not then "Err" is yielded and the loop will restart back to the top.
    };
    
    //Print our number from the buffer we inputted.
    println!("You gussed: {guess}");

        match guess.cmp(&secret_number) { //comparing our guess to the secret_number.
            Ordering::Less => println!("Too small"), //The ordering pairs are just enums like in c++. This is very useful. I like rust for this. If any of these two are a match the program will go back to the loop
            Ordering::Greater => println!("Too big"), //so basically these are just checking if our guess is less than, greater than, or equal to the secret_number value.
            Ordering::Equal => { 
                println!("You win!"); //We win
                break; //loop exits, program exits.
            }
        }   
    }
}
