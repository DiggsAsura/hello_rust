/*  

loop

Need an infinite loop?

Rust makes it easy.

break will escape a loop when you are ready.

loop has a secret we'll talk about soon.

*/

fn main() {
    let mut x = 0;
    loop {
        x += 1;
        if x == 42000 {
            break;
        }
        println!("{}", x)
    }
    println!("{}", x);
}