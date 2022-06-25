// Trying to get into taking inputs in Rust. 
// It's so freaking conveluted!

use std::io::stdin;

fn main() {
    println!("Something something: ");
    let mut name = String::new();
    stdin().read_line(&mut name)
        .ok()
        .expect("Failed");
    
    match &*name.trim() {
//    match name.as_str().trim() {
        "Diggs" => println!("Cool as heck eh!"),
        "Sarah" => println!("Best step daughter!"),
        "Tetris" => println!("Hell yea"),
        _ => println!("No match"),
    };
}
