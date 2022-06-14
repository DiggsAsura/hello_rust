// modules
// allows you to separate your code to organised sections

mod kenny {
    // private 
    fn chicken() { // this is not callable from outisde.. unless... see below
        println!("Chicken");
    }
    pub fn print_msg() {
        println!("How's it going!");
        chicken();  // ...calling this works, when called from within the pub fn
    }

    // nested modules
    
    pub mod water {
        pub fn print_msg() {
            println!("I'm water");
        }
    }
}

// will fail if mod function above not is public (pub)
fn main() {
    kenny::print_msg();
    kenny::water::print_msg();
}


// this is awesome!
