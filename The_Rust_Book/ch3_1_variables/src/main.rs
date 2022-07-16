// Redoing this as its a long time since last time. 

fn main() {
    let mut x = 5;
    // OMFG can write {variable} wtf!
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is now: {x}");
    hours();
    shadow_galore();
}

// CONSTANTS
// let vs const
// const is always immutable, can not do it mutable at all
// constants can not be used when the result is calculated at runtime
// can be used in global scope
// CONST naming convention is all uppercase
// Constants are valid for the enitre time the program runs

// Notice this is done in global scope
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// Does NOT work
// let msg = "Hi";
fn hours() {
    let msg = "Hi";
    println!("{msg}! Three hours in seconds: {THREE_HOURS_IN_SECONDS}");
}

// SHADOWING
// We already did it in the beginning. When changing/shadowing x into 6. 
// Let's do a couple more.

fn shadow_galore() {
    let x = 5;
    let x = x + 1; // works because new let
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x in this function shadow_galore(), is: {x}");
    
    let spaces = "     ";
    let spaces = spaces.len();
    println!("The length of spaces is {spaces}");
}
