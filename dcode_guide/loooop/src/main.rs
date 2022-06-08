fn main() {
    let mut n = 0;

    loop {
        n += 1;
        if n > 2000 {
            break;
        }
        println!("The value of n is {}", n);
    }
}
