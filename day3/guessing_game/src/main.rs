// Gonna break it down with 
// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html


// Bring the io (input/output) library into scope, from the standard library.
// Similar to python import i guess?
// ..like..
// from std import io
// ..i assume ?
//use std::io;


// the fn main() will be in every program. An entry function. 
// fn declares a new function
// () means theres no parameters
// everything inside {}
//fn main() {

    //println! is a macro that prints to the screen (the ! makes it for some reason)
//    println!("Guess the number!");
//    println!("Please input your guess.");

    // Storing Values with Variables
    // Next, we'll create a variable to store the user input, like this
//    let mut guess = String::new();

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
//    io::stdin()
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
//        .read_line(&mut guess)


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
//        .expect("Failed to read line");
    

    // {} are also placeholders, similar to python really
    // let x = 5;
    // let y = 10;
    // println!("x = {} and y = {}", x, y)
//    println!("You guess is: {}", guess);
//}

// This is really in Day 4, but yea continue in the same folder for convenience



// Next up is making the random, secret number.
// apparently Rust does not have it included in the standard library, but we can add it to the 
// dependencies (i guess). rand create.

// Using a Create to get more Functionality
// Remember that a create is a colleciton of Rust source code files. The project we've been building
// is a binary crate, which is an executable. The rand create is a library crate, which contains code
// intended to be used in other programs, and can'b be executed on its own.

// Cargo's coordination of external crates is where Cargo really shines. Before we can write code that
// uses rand, we need to modify the Cargo.toml file to include the rand crate as a dependency. Open
// that file now and add the following line to the [dependencies] section header that Cargo created for
// you. Be sure to specify rand exactly as we have here, with this version number, or the code examples
// in this tutorial may not work.

// rand = "0.8.3"


// Generating a Random number
// (commented out everything to write the new modified version here)

//use std::io;
//use rand::Rng;
//
//fn main() {
//    println!("Guess the number!");
//
//    let secret_number = rand::thread_rng().gen_range(1..101);
//
//    println!("The secret number is {}", secret_number);
//    println!("Please input your guess.");
//
//    let mut guess = String::new();
//    io::stdin()
//        .read_line(&mut guess)
//        .expect("Failed to read line");
//    
//    println!("You guessed: {}", guess)
//}

// rand::thread_rng
// The gen_range method takes a range expression as an argument and generates a random number in the range. 
// start..end is inclusive on the lower bound, but exclusive on the upper bound, so we need to 
// specify 1..101 to request a number between 1 and 100. Alternatively, we could pass the range
// 1..=100, which is equivalent. 

//
// Note
// You won't just know which traits to use and which methods and functions to call from a crate, so
// each crate has documentation with instructions for using it. Another neat feature of Cargo is
// that running the cargo doc --open command will build documentation provided by all of your 
// dependencies locally and open it in your browser. If you're interested in other functionality 
// in the rand crate, for example, run cargo doc --open and click rand in the sidebar on the
// left.

// The second new line prints the secret number. This is useful while we're devloping the program
// to be able to test it, but we'll delete it from the final version. It's not much of a game if the
// program prints the answer as soon as it starts!

// ----------------------
// ----------------------


// COMPARING THE GUESS TO THE SECRET
// same, ill comment out everything and revwiret all in this section.


//use rand::Rng;
//use std::cmp::Ordering;
//use std::io;

//fn main() {
    // --snip--
//    println!("Guess the number!");

//    let secret_number = rand::thread_rng().gen_range(1..101);

//    println!("The secret number is: {}", secret_number);

//    println!("Please input your guess.");

//    let mut guess = String::new();

//    io::stdin()
//        .read_line(&mut guess)
//        .expect("Failed to read line");


    // added after the beneth text about changing string to number
//    let guess: u32 = guess.trim().parse().expect("Please type a number!");

//    println!("You guessed: {}", guess);

//    match guess.cmp(&secret_number) {
//        Ordering::Less => println!("Too small!"),
//        Ordering::Greater => println!("Too big!"),
//        Ordering::Equal => println!("You win!"),
//    }
//}

// this does not seem to compile yet. 

// std::cmp::Ordering
// brought in from the standard library. cmp, compare to values. Ordering is brought into scope. 
// Ordering has Less, Greater and Equal.

// A match expression is made up of "arms". An arm consists of a pattern to match against, and the code
// that should be run if the value given to match fits the arm's pattern. Rust takes the value given to
// match and looks through each arm's pattern in turn. Patterns and the match construct are powerful
// Rust features that let you express a variety of situations your code might encounter and make sure
// that you handle them all. These features will be covered in detail in Chapter 6 and Chapter 18.

// need to deine String to be integer
// default i32 type integers but there is also u32 (unsigned), i64 and u64
// secret_number is defaulted to i32 unless other specified 

// added let guess: u32 = guess.trim().parse().expect("Please type a number!");
// pretty cool, its called shadowing and covered more in the next chapter
// it will let us add to the same variable guess without overwriteing

// guess.trim().parse().
// The guess in the expression refers to the original guess variable that contained the input as a
// string. The trim method on a String instance will eliminate any whitespace at the beginning and end,
// which we must do to be able to compare the string to the u32, which can only contain numerical data.
// The user must press enter to satisfy read_line and input their guess, which adds a newline character
// to the string. For example, if the user types 5 and presses enter, guess looks like this: 5\n. The
// \n represnts "newlin". (On Windows, pressing enter results in carriage rturn and a new newline, 
// \r\n). The trim method eliminates \n or \r\n, resulting in just 5


//
//
// ALLOWING MULTIPLE GUESSES WITH LOOPING
//
//

// The loop keyword creates an infinite loop. We'll add a loop to give users more chances at 
// guessing the number:

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..101);
    //println!("The secret number is: {}", secret_number);
    // literally removed to not blow the game 

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // changed to not panick the program if enter non-number.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            // notice the {}
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}