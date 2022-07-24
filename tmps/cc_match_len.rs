use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error");

    let input_len = input.trim().len();

    println!("{input_len}");

    match input_len {
        1..=4 => println!("Less then or equal to 4: {input_len}"),
        5..=10 => println!("Between 5 and 10: {input_len}"),
        _ => println!("Blarg, no match! {input_len}"),
    };
}


