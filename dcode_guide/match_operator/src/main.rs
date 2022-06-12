fn main() {
    let number = 11;

    match number {
        1 => println!("It is one!"),
        2 => println!("It is two!"), // 2..20 range is experiemental or deprecated ...
        10 | 11 => println!("The answer is 10 or 11"),
        _ => println!("It doesn't match!") // _ does not match any
    }

    let name = "Kenneth";

    match name {
        "Chris" => println!("Nice name, mate!"),
        "Domenic" => println!("Allright"),
        "Kenneth" => println!("Dam nice choice"),
        _ => println!("Neither")
    }
}
