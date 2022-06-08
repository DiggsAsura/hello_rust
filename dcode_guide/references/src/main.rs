// another way to refer to a variable

fn main() {
    //dominic
    // reference
    //dom
    //
    // & is a reference
    let mut x = 10;
    
    // dom is now a mutable reference to x
    // let us change x through dom
    {
      let dom = &mut x;
      // * needed. dont remember why though lol. 
      *dom += 1;
    }

    println!("x is {}", x);
}

// a bit confused. guide says i need a code-block to print x, but i did not. did rust
// change since the guide was made?
