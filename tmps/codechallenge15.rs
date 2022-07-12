// Vacation
// Still learning, but no time. Keeping streak
//
use std::io::stdin;

fn main() {
    // Trying this again..
    let mut name = String::new();
    stdin().read_line(&mut name)
        .ok()
        .expect("Fail");
    println!("Hello, {}", name.trim());
}
