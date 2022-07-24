use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let yesno = rng.gen_range(0..2);

    match yesno {
        0 => println!("Yes! {yesno}"),
        1 => println!("No!! {yesno}"),
        _ => println!("Something went wrong.. {yesno}"),
    };
    

}
