// not sure if i really understand this one. I mean, the usecase. 

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); //makes a new vector that holds strings

//    for argument in args.iter() {
//        println!("{}", argument);
//
    println!("{}", args[1]);
}
