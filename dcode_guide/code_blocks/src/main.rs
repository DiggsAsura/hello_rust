// code block is a piece of code that is found within { }
// isolated, but has access to things outside the brackets. 
// but not the other way around
//

fn main() {
    let x = 10;

    {
        let y = 5;
        // this one works
        println!("x {}, y {}", x, y);
    }


    // this will not work, y is outside of scope!
//  println!("x {}, y {}", x, y);
}
