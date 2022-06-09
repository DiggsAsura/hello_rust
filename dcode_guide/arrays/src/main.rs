fn main() {
    //let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let numbs = [2; 400];
    
    //numbers[0] // 1
    //numbers[4] // 4

    for n in 0..numbs.len() {
        println!("{}", numbs[n]);
    }
}
