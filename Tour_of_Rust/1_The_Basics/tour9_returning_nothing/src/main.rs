/*

Returning Nothing

If no return type is specified for a function, it returns an empty tuple, also known
as a unit.

An empty tuple is represented by ().

Using () is uncommon, but will come up often enough that it's good to know whats
happening.

*/

fn make_nothing() -> () {
    return ();
}

// the return type is implied as ()
fn make_nothing2() {
    // this function will return () if nothing is specified to return
}

fn main() {
    let a = make_nothing();
    let b = make_nothing2();

    // Printing a debug string for a and b
    // Because it's hard to print nothingness
    println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);
}