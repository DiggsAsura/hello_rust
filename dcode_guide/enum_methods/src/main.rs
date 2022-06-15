// defining your own methods/funcions on enum types in rust
// allows you to move some of your logic from the main functions into the enum types itself
// to clean up code and less confusing!

#![allow(dead_code)]

enum Day {
    Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
}

impl Day {
    // a function for the Day enum
    fn is_weekday(&self) -> bool {
        match self {
            &Day::Saturday | &Day::Sunday => return false, // if the day is saturday or sunday, return false
            _ => return true
        }
    }
}

fn main() {
    let d = Day::Tuesday;
    let d2 = Day::Saturday;
    println!("Is d a weekday? {}", d.is_weekday());
    println!("Is d2 a weekday? {}", d2.is_weekday());

}
