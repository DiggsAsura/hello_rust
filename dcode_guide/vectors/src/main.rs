// vector is a beefed up array on steroids
// its this data structure where you can store a sequence of elements inside 

fn main() {
    //let my_vector: Vec<i32>  // <> whats stored inside!
    // more normal way though:
    let mut my_vector = vec![1, 2, 3, 4];

    // print 3
    println!("{}", my_vector[2]);

    // append to vector
    my_vector.push(49);
    // remove from vector
    my_vector.remove(1); // remove '2'

    for num in my_vector.iter() {  // use .iter() to not loose the elements inside
        println!("{}", num);
    }
}
