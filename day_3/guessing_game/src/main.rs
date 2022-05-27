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



    // RECEIVING USER INPUT
    // recall use std:io;
    // now we call the stdin() function from the io module
    // which allows us to handle input from users
    io::stdin()
    // if we didnt import io library at the beginning (use std:io) we could still do it by writing
    // std:io::stdin. The stdin function returns an instance of std:io:Stdin, which is a type that
    // represents a handle to the standard input for your terminal.

        // Next, the line .read_line(&mut guess) calls the read_line method on the standard input handle
        // to get input from user. We're also passing &mut guess as argument to read_line to tell it what
        // string to store the user input in. The full job of read_line is to take whatever the user types
        // into standard input and append that into a string (without overwriting its contents), so we therfor
        // pass that string as an argument. The string argument needs to be mutable so the method can change
        // the string's content.

        // the & indicates this argument is a reference, which gives you a way to let multiple parts of your
        // code access one piece of data without needting to copy that data into memory multiple times.
        // refrences are immutable by default. Hence you write &mut guess rahter then &guess to make it
        // mutable.
        .read_line(&mut guess)


        // Handling Potential Failure with the Result Type
        // This last part is the .expect method. We broke this up in several lines, but could be
        // written like this:
        // io::stdin().read_line(&mut guess).expect("Failed to read line");
        // but divided it for ease of use/read

        // read_line puts whatever the user enters into the string we pass to it, but it
        //           also returns a value-in this case, an io::Result.
        // theres two Result varians, Ok and Err. 
        // The expect is somewhat expected by the compiler too deal with the situation. So if it fails
        // (Err) it will print the following line. If not, the compiler gives us a warning.
        .expect("Failed to read line");
    

    // {} are also placeholders, similar to python really
    // let x = 5;
    // let y = 10;
    // println!("x = {} and y = {}", x, y)
    println!("You guess: {}", guess);
}
