/*
Functions

A function has zero or more parameters.

In this example, the add function takes two arguments of type i32 (signed integer of 
32-bit length).

If you just want to return an expression, you can drop the return keyword and the
semicolon at the end, as we did in the subtract function.

Function names are always in snake_case.

Hint: if you define a function, the data it accepts are called parameters. If you call
that function and pass data to it, then it's called arguments.

*/

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y // dropping return and ;
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn divide(x: f32, y: f32) -> f32 {
    x / y
}

fn main() {
    println!("42 + 13 = {}", add(42, 13));
    println!("42 - 13 = {}", subtract(42, 13));
    println!("42 * 13 = {}", multiply(42, 13));
    println!("42 / 13 = {}", divide(42.0, 13.0));
}
