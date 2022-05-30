/*
Calling Methods

Unlike functions, methods are functions associated with a specific data type.

static methods - methods that belong to a type itself are called using the
:: operator. 

instance methods - methods that belong to an instance of a type are called using
the . operator.

We will talk more on making your own methods in future chapters.

*/

fn main() {
    // Using a static method to create an instance of String
    let s = String::from("Hello World!");
    // Using a method on the instance
    println!("{} is {} characters long.", s, s.len());
}