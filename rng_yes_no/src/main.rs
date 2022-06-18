use rand::seq::SliceRandom;

fn main() {
    let values = vec!["yes", "no"];
    println!("{:?}", values.choose(&mut rand::thread_rng()));
}
