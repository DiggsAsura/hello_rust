//not built in

extern crate rand;
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1, 110); // 1-10 inclusive
    println!("Random Number: {}", random_number);

    // Flip a coin
    let random_bool = rand::thread_rng().gen_weighted_bool(2); // 1 in 2
    println!("50/50: {}", random_bool)

}
