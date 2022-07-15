//
// Chap 3.3 - Functions
//

// Naming convention: snake case for function and variable names (lower_case)

fn main() {
    println!("I'm in the main() function");
    first_function();
    second_function(120);
    third_function(5, 'h');
}

fn first_function() {
    println!("I'm from another function.");
}

// Define a new function with the fn followed by the function name and a set of 
// parantheses. The curly brackets tell the compiler where the function body begins and
// ends. 
//
// Take note that another_function() is defined after it's called. Rust does not care,
// as long as it's in a scope that can be seen by the caller.


// Parameters
// ------------

fn second_function(x: i32) {
    println!("The value of x is {x}");
}
// In function signatures, you MUST declare the type of each parameter. This is a deliberate
// decision in Rust's design: requiring type annotations in function definitions means the
// compiler almost never needs you to use them elsewhere in the code to figure out what type
// you mean. The compiler is also able to give more helpful error messages if it knows what
// types the function expects. 

fn third_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}



// Statments and Expressions
// ============================
