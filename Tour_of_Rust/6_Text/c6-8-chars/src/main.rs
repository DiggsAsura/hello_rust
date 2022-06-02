fn main() {
    // collect the characters as a vector of a char
    let chars = "hi c".chars().collect::<Vec<char>>(); // c should be an emoji, but suddently stopped working Oo
    println!("{}", chars.len()); // should be 4 (with emoji as c)
    // since chars are 4 bytes we can convert to u32
    println!("{}", chars[3] as u32);
}