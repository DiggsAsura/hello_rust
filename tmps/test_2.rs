fn main() {
    let hex_test = 0xff;
    println!("{hex_test}");
    // OMG it was 255 haha. 
    // Ok time to learn HEX? xD
    
    #[allow(overflowing_literals)]
    // ok a bit fun, this does not work
    // as i8, as the range is from -128 to 127.
    // but with [allow] it does, but returns -1 lol
    let hex_test_2: i8 = 0xff;
    println!("Drumrull: {hex_test_2}");

    // Heres a real one
    let hex_test_3: u8 = 0xff;
    println!("Hex: 0xff = {hex_test_3}");
}
