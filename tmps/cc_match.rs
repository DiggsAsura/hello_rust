fn main() {
    let a = 1;
    let b = 1;
    
    let my_result = add(a, b);

    match my_result {
        1 => println!("The result is 1"),
        2 => println!("The result is 2"),
        3 => println!("The result is 3"),
        _ => println!("Out of bounds"),
    };

}


fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
