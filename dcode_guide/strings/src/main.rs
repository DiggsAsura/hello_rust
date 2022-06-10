// Strings is complicated in rust
// Primitive type and the main string datatype
// This is main string datatype

fn main() {
    let mut my_string = String::from("How's it going? My name is Dom.");

    // length
    println!("Length: {}", my_string.len());
    // Is Empty?
    println!("String is empty? {}", my_string.is_empty());

    // token can be whatever i think
    for token in my_string.split_whitespace() {
      println!("{}", token);
    }

    // check if contains
    println!("Does the string contain 'Dom'? {}", my_string.contains("Dom"));

    // need string variable mutable. append to string
    my_string.push_str(" Welcome to your tutorisal on Strings!");

    println!("{}", my_string);
}
