/*

while

lets you easily add a condition to a loop.

if the condition evaluates to false, the loop will exit.

*/

fn main() {
    let mut x = 0;
    while x != 42 {
        x += 1;
        println!("{}", x)
    }
    println!("x is {}", x);
}