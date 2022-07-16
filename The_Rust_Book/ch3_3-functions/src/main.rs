//
// Chap 3.3 - Functions
//

// Naming convention: snake case for function and variable names (lower_case)

fn main() {
    println!("I'm in the main() function");
    first_function();
    second_function(120);
    third_function(5, 'h');
    statement();
    expression();
    
    //let x = five();
    println!("{:?}", five());

    println!("{}", plus_one(6));
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
// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to resulting value. 
//
// We already used statements and expressions. Creating a variable and assigning a value 
// to it with the let keyword is a statement:

fn statement() {
    let y = 6;
    println!("statement: let y = {y}");
    println!("But can not like other langugaes do: x = y = 6, where both x and y is 6");
    // Does not work
    // let x = (let y = 6);
    // The let y = 6 statement does not return a value, so there isn't anything for x to
    // bind to. This is different from what happens in other languages, such as C, Ruby, Python
    // etc where the assignment returns the value of the assignment. In those languages, you
    // can write x = y = 6 and have both x and y have the value 6.
    //
}
    // Expressions evaluate to a value and make up most of the rest of the code tha you'll
    // write in Rust. Consider a math operation, such as 5 + 6, which is an expression that
    // evaluates to the value 11. Expressions can be part of statements: below, the 6 in the
    // statement let y = 6; is an expression that evaluates to the value 6. Calling a funciton 
    // is an expression. Calling a macro is an expression. A new scope block created with 
    // curly brackets is an expression for example: 
fn expression() {
    let y = {        // Expression follows
        let x = 3;
        x + 1        // Don't have; !
    };
    println!("The value of y is {y}");
    
    // x + 1 does not have ending semicolon. If you add semicolon, you turn it into a 
    // statement and will then not RETURN a value. 
}


// Functions with Return Values
// ==============================

// Functions can return values to the code that calls them. We don't name reutrn values, but
// we must declare their type after an arrow (->). In Rust, the return value of the function
// is synonymous with the value of the final expression in the block of the body of a 
// function. You can return early from a function by using the return keyword and spcifying
// a value, but most functions return the last expression implicitly. Here's an example
// of a function that returns a value: 

fn five() -> i8 {
    -5
}

fn plus_one(x: u32) -> u32 {
    x + 1 
}

// If adding x + 1; it will fail in compilation. It's now a statement, not an expression. And 
// mismatch in types. 
