use std::io::stdin;

fn main() {
    println!("What's your name: ");
    let mut name = String::new();
    stdin().read_line(&mut name)
        .ok()
        .expect("Failed");
    
    println!("Your name is {}", name);
}
