fn main() {
    // Data Types
    let guess: u32 = "42".parse().expect("Not a number!");
    let sum = guess + guess;
    println!("{guess} + {guess} = {sum}");

    // Number Literals
    let decimal = 98_222; // _ will still be 98222 
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A'; // u8 only

    println!("Decimal: {decimal}");
    println!("Hex: {hex}");
    println!("Octal: {octal}");
    println!("Binary: {binary}");
    println!("Byte (u8 only): {byte}");

    
    // Floating-Point Types
    let float_1_32: f32 = 0.1;
    let float_2_32: f32 = 0.2;
    let float_3_32: f32 = float_1_32 + float_2_32;

    let float_1_64: f64 = 0.1; // f64 default
    let float_2_64 = 0.2;
    let float_3_64 = float_1_64 + float_2_64;

    println!("Float 32: {float_1_32} + {float_2_32} = {float_3_32}");
    println!("Float 64: {float_1_64} + {float_2_64} = {float_3_64}");

    
    // The Character Type
    // Rust's char type is the language's most primitive alphabetic type. Here's some 
    // examples of declaring char values:

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c: {c}");
    println!("z: {z}");
    println!("Emoji: {heart_eyed_cat}");

}
