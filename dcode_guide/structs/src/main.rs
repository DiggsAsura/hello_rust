// structs is a way to group similar bits of information into one central datatype

struct Color {
    red: u8, // u8: supports 0-255 which is super for colors!
    green: u8,
    blue: u8
}

fn main() {
    let mut bg = Color { red: 255, green: 70, blue: 15 };

    // to change bg.blue, need to do let bg mut above

    bg.blue = 34; 
    println!("{} {} {}", bg.red, bg.green, bg.blue);
}
