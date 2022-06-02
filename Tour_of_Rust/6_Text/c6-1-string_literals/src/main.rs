fn main() {
    let a: &'static str = "hi 🦀";
    println!("{} {}", a, a.len());
}
