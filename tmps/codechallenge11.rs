fn subtract(a: u32, b: u32) -> u32 {
    a - b
}

fn main() {
    let num_a = 100;
    let num_b = 12;
    let sum = subtract(num_a, num_b);

    println!("{}-{}={}", num_a, num_b, sum);
}
