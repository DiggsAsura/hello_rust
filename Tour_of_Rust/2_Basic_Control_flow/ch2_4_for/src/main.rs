/*

for

Rust's for loop is a powerful upgrade. It iterates over values from any expression that
evaluates into an itereator. What's an iterator? An iterator is an object that you can
ask the question "What's the next item you have?" until there are no more items.

We'll explore this more in a future chapter. In the meantime, just know Rust makes
it easy to create iterators that generate a sequence of integer numbers.

The .. operator creates an iterator that generates numbers from a start number up to 
but not including an end number.

The ..= operator creates an iterator that generates numbers from a start number up to 
and including an end number.

*/

fn main() {
    for x in 0..5 {
        println!("{}", x);
    }

    for x in 0..=5 {
        println!("{}", x);
    }
}