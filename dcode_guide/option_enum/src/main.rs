// option represents a value, or no value at all
// many methods returns this type, and force you to deal with it


fn main() {
    let name = String::from("KennethBB");

    // index 8 does not exist

    println!("Character at index 8: {}", match name.chars().nth(8) {   // name.chars().nth(8) returns an option
        Some(c) => c.to_string(),
        None => "No character at index 8!".to_string()
    });

    println!("Occupation is {}", match get_occupation("Kenneth") {
        Some(o) => o,
        None => "No occupation found"
    });
}

fn get_occupation(name: &str) -> Option<&str> {   // when we do get a value from this function, it will be the type string (if the Some is returned)
    match name {
        "Kenneth" => Some("Software Developer"),
        "Michael" => Some("Dentist"),
        _ => None
    }
}
