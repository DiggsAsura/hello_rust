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
}


// Handling Multiple Conditions with else if
// -------------------------------------------


