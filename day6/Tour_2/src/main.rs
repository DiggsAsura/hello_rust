// Rust cares a great deal about what variables are modifiable. Values fall into
// two types:
//
// mutable - the compiler will allow the variable to be written and read from
//
// immutabel - the compiler will only allow to be read from
//
// Mutable values are denoted with a mut keyword.
//
// We will have more to say on this concept later, but for now just keep an
// eye out for this keyword.

fn main() {
    let mut x = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);
}

