// a tuple is a bunch of variables in on collection

fn main() {
    // can do a lot of data types, including tuples for nested tuples
    let tup1 = (20, "Yolo", 3.14, "Rust", (1, 2, 3, 4));
    
    // to access the third item in the tuple
    println!("{}", tup1.2);

    // to access item inside the nested tuple
    println!("{}", (tup1.4).2);
    
    // extract all the items of the tuples into separate variables
    let (a, b, c, d, e) = tup1;

    println!("{:?} {:?} {:?} {:?} {:?}", a, b, c, d, e);

}
