// Chapter 3.5 - Control Flow
// ============================
// if expressions and loops


// if expressions 
// ----------------
fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    // if not providing else, and the condition is false, the program will just skip
    // the if block/arm and move to the next bit of code
    } else {
        println!("Condition was false");
    }

    // it's also worth noting that the condition in this code MUST be a bool. If the condition
    // isn't a bool, we'll get an error. For example, try running the following code:
    //if number {
    //    println!("number was {number}");
    //}

    // Nope, above won't run. 
    // The error indicates that Rust expected a bool but got an integer. Unlike languages
    // such and Ruby and JavaScript, Rust will not automatically try to convert non-Boolean
    // types to a Boolean. You must be explicit and always provide if with a Boolean as
    // its condition. If we want the if code block to run only when a number is not equal
    // to 0, for example, we can change the if expression to the following:

    if number != 0 {
        println!("number was something other than zero");
    }

    // 123 Following parts
    part_two(12);
    part_three();
    part_four();
}


// Part 2: Handling Multiple Conditions with else if
// --------------------------------------------------
// example handle if/else if/else

fn part_two(number: i32) {
    //let number: i32 = 6;
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0{
        println!("number is divisible by 2");
    } else {
        println!("number is not devisible by 4, 3 or 2");
    }
}
// Using too many if/else can clutter the code. We'll look at match later for that. 


// Part 3: Using if in a let Statement
// ----------------------------------------
// Because if is an expression, we can use it on the right side of a let statement to 
// assign the outcome to a variable:

fn part_three() {
    let condition = true; // 5, false becoming 6
    let number = if condition { 5 } else { 6 };

    println!("The value ofnumber is: {number}");
}
// let number = if condition { 5 } else { six }
// This will fail, because variables need one type only. Means both arms need to be same type
// or it will fail at compile time. 



// Part 4: Repeating Code with loop
// -----------------------------------

// The loop keyword tells Rust to execute a block of code over and over again forever
// until you explicitly tell it to stop. 
//
// As an example, try this. Ctrl-c to stop:

fn part_four() {
    let mut count = 0;
    loop {
        if count < 100 {
            println!("Count: {count}");
            count += 1;
            continue
        } else {
            break
        } 
    }
}
