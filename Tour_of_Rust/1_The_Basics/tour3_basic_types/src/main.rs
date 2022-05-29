/*
Basic Types
Rust has a variety of familiar types:

- booleans - bool for representing true/false

- unsigned integers - u8, u16, u32, u64 and u128 for representing nonnegative whole numbers

- signed integers - i8, i16, i32, i64, i128 for representing whole numbers

- pointer sized integers - uzise isize for representing indexes and sizes of things in memory

- floating point - f32, f64

- tuple - (value, value, ...) for passing fixed sequences of values on the stack

- arrays - [value, value, ...] a collection of similar elements with fixed length known at compile time

- slices - a collection of similar elements with length known at runtime

- str (string slice) - text with a length known at runtime


Text might be more complex than you are used to in other languages; since Rust is a
system programming language, it cares about memory issues you might not be used to. 
We will be going into this in detail later.

Numberic types can be explicitly specified by appending the type to the end of the
number (13u32, 2u8..)

*/

fn main() {
    let x = 12; // by default this is i32
    let a = 12u8;
    let b = 4.3; // by default this is f64
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let sentence = "Hello World!";
    println!("{} {} {} {} {} {} {} {}", x, a, b, c, bv, t.0, t.1, sentence);
}

