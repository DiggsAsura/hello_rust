// Farenheit to Celcius converter
use std::io;

fn main() {
    let mut degree = String::new();

    println!("Welcome to F/C converter.\nPlease input your value:");
    io::stdin().read_line(&mut degree).expect("Error");

    // THIS LINE <3 <3 
    // IS 
    // AWESOME. 
    // or at least usefule! Changing the user input type to f32. Thanks god, so hard.
    let degree: f32 = degree.trim().parse().unwrap();
    let cel_to_far = c_to_f(degree);
    let far_to_cel = f_to_c(degree);

    println!("\n{degree}C = {cel_to_far}F");
    println!("{degree}F = {far_to_cel}C\n");
}

fn c_to_f(c: f32) -> f32 {
    c * 1.8 + 32.0
}

fn f_to_c(fa: f32) -> f32 {
    (fa - 32.0) / 1.8
}
