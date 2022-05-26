// Gonna break it down with 
// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html


// Bring the io (input/output) library into scope, from the standard library.
// Similar to python import i guess?
// ..like..
// from std import io
// ..i assume ?
use std::io;


// the fn main() will be in every program. An entry function. 
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guess: {}", guess);
}
