// shadowing is reassigning variables, but not really...?

fn main() {
    let mut x = 10;

    // random codeblock
    {
      // shadowing (inside the codeblock?) with let (without wont work
         let x = 15;
         println!("{}", x);

         // do stuff with 15
    }
    println!("the original x {}", x);


    // change data type of x
    let x = "X is now a string";
    println!("{}", x);


    // change to bool
    let x = true;
    println!("{}", x);

}


// not really sure whats going on with the original, besides that we keep reassigning.
