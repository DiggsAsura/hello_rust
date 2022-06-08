fn main() {
    print_numbers_to(200);
/*
    if is_even(39) {
        println!("It is even!");
    } else {
        println!("It is odd!");
    }
*/
}

// looks like the order of functions does not matter?
fn print_numbers_to(num: u32) {
    for n in 1..num {
        if is_even(n){
            println!("{} is even", n);
        } else {
            println!("{} is odd", n);
        }
    }
}

// the arrow is defining what data type the function is returning
fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}
