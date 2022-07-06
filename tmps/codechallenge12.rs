// Trying again to take input as integer only

use std::io::*;
//use std::fs::read;

fn read(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}

fn main() {
    println!("Will this take my inputs as ints?");

    let mut num1 = String::new();
    read(&mut num1);
    let mut num2 = String::new();
    read(&mut num2);

    let num1: i32 = num1.trim().parse().unwrap();
    let num2: i32 = num2.trim().parse().unwrap();

    let sum = num1 + num2;

    println!("{} + {} = {}", num1, num2, sum);
}

// Kinda sorta did, but not really. I copied a function from an earlier project
// but I still don't really understand it.


