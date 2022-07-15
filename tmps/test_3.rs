fn main() {
    // Number literals
    let octal_1 = 0o77;
    let hex_1 = 0xff;
    let decimal_1 = 98_222; // _ can be used, this is still 98222 
    let binary_1 = 0b1111_0000; 
    let byte_1: u8 = b'A';

    // test
    let byte_2: u8 = b'B';
    let comb_byte: u8 = byte_1 + byte_2;

    // the aboves.. good to know.
    println!("Decimal: {decimal_1}");
    println!("Hex: {hex_1}");
    println!("Octal: {octal_1}");
    println!("Binary: {binary_1}");
    println!("Byte (u8 only): {byte_1}");

    println!("{byte_1} + {byte_2} = {comb_byte}");
}
