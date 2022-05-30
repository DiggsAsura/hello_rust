/* 
match

Miss your switch statement? Rust has an incredibly useful keyword for matching all
possible conditions of a value and executing a code path if the match is true.
Let's see how this works for numbers. We will have more to say in future chapters
on pattern matching more complex data. I promise you it will be worth the wait.

match is exhaustive so all cases must be handled.

Matching combined with destructuring is by far one of the most common patterns you 
will see in all of Rust. 

*/

fn main() {
    let x = 42;

    match x {
        0 => {
            println!("found zero");
        }
        // we can match against multiple values
        1 | 2 => {
            println!("found 1 or 2!");
        }
        // we can match against ranges
        2..=9 => {
            println!("found a number 3 to 9 inclusive");
        }
        // we can bind the matched number to a variable
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        // this is the default match that must exist if not all cases are handled!
        _ => {
            println!("found something ELSE! Nice, so thats what _ means :D")
        }
    }
}