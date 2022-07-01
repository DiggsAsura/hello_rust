// Try to take user input, but as int only
use std::io::stdin;

fn main() {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
        .ok()
        .expect("Fail");

    let user_val = match input_string.parse::<i32>() {
        Ok(x) => x,
        Err(_) => -1,
    };
}
