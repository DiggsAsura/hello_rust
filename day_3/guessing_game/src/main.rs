// Gonna break it down with 
// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html


// Bring the io (input/output) library into scope, from the standard library.
// Similar to python import i guess?
// ..like..
// from std import io
// ..i assume ?
use std::io;


// the fn main() will be in every program. An entry function. 
// fn declares a new function
// () means theres no parameters
// everything inside {}
fn main() {

    //println! is a macro that prints to the screen (the ! makes it for some reason)
    println!("Guess the number!");
    println!("Please input your guess.");

    // Storing Values with Variables
    // Next, we'll create a variable to store the user input, like this
    let mut guess = String::new();

    // above here a lot of stuff going on
    // let statement creates the variable
    // another example:
    // let apples = 5; 
    // (assume that is like python: apples = 5)

    // This line above creates the variable apples and binds it to have the value 5. In Rust,
    // variables are immutable by default. We'll be discussing this concept in detail in the
    // "Variables and Mutability" section in Chapter 3. To make a variable mutable, we add mut
    // before the variable name:

    // let apples = 5; // immutable
    // let mut bananas = 5; //mutable

    // back to the actual program. Now we know that we just made a mutable variable, guess.
    // the guess variable is equal to a new string.. 
    
    // String::new();
    // This will return a new instance of a String. String is a string type provided by the standard
    // library that is growable, UTF-8 encoded bit of text.

    // The :: syntax in the ::new line indicates new is an associate function of the String type. An 
    // associated function is a function that's implemented on a type, in this case String. This new
    // function creates a new, empty string. You'll find a new function on many types, because it's 
    // a common name for a function that makes a new value of some kind.

    // In full, the let mut guess = String::new() line has created a mutable variable that is currently
    // bound to a new, empty instance of Sring. Whews!




    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guess: {}", guess);
}
