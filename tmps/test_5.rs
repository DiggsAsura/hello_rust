// Never can't seem to rember how to take basic input! How hard can it be lol!

use std::io;

fn main() {
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .ok()
        .expect("Failed");

    println!("Hello {name}");
}
