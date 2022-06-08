// constants are variables which are declared in the global scope and can not be changed
// (immutable?)

// naming convention:
// MAXIMUM_NUMBER

// with consts we need to specify the datatype
const MAXIMUM_NUMBER: u8 = 20;

fn main() {
    for n in 1..MAXIMUM_NUMBER {
        println!("{}", n);
    }
}
