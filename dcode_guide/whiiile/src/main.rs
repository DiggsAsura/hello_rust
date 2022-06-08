fn main() {
    let mut n = 1;

    while n <= 200 {
        if n % 50 == 0 {
            // if n is a multiple of 5
            println!("n is {}", n);
        }
        n += 1;
    }

}
