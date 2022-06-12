// HashMaps 
// A collection of key value pairs. Dictionaries in Python??

use std::collections::HashMap;

fn main() {
    // key => value
    // you ask for the key, and get the value
    let mut marks = HashMap::new();

    // Add Values
    // .insert requires two arguments
    marks.insert("Rust Programming", 96);
    marks.insert("Web Development", 94);
    marks.insert("UX Design", 75);
    marks.insert("Professional Computing Studies", 45);


    // Find length of HashMap
    println!("How many subjects have you studied? {}", marks.len());


    // Get a single value
    match marks.get("Web Development") {
        Some(mark) => println!("You got {} for Web Dev!", mark),
        None => println!("You didn't study Web Development")
    }

    // Remove a value
    marks.remove("Ux Design");

    // Loop through HashMap
    for (subject, mark) in &marks {
        println!("For {} you got {}%", subject, mark);
    }

    // Check for value
    println!("Did you study C++? {}", marks.contains_key("C++ Programming"));
}
