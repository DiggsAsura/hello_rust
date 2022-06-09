// tuple structs
// a tupled struct does not have the name of the members, but just the types (inside a tuple)


struct Color(u8, u8, u8); // three members, and all u8

fn main() {
    let mut red = Color(255, 0, 0);

    red.2 = 60;

    println!("red is {}, {}, {}", red.0, red.1, red.2);

}
