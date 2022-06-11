use std::fs::File; //struct for the files
use std::io::prelude::*; //help us do read and write operations to files

fn main() {
    let mut file = File::open("text.txt").expect("Can't open file!");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Ooops! Can not read the file...");

    println!("File Contents:\n\n{}", contents);
}
