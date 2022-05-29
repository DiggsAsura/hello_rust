fn main() {
    // rust infers the type of x
    let x = 13;
    println!("{}", x);

    // rust can also be explicit about the type
    let x: f64 = 3.14159;
    println!("{}", x);

    // rust can also declare and initialize later, but is rarily done
    let x;
    x = 0;
    println!("{}", x);
}