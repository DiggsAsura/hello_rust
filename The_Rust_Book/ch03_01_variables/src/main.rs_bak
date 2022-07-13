// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

fn main() {
    // immutable by default
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The new value of x is: {}", x);

    // Consts will always be immutable. We declare consts with consts instead
    // of mut. And it MUST be annotated. (?!?!?!)
    // Last difference is that constants may be set to only to a constant expression,
    // not the result of a value taht could only be computated at runtime. Not sure 
    // what this even means.. lol.
    // Rusts naming conventions is ALL_UPPERCASE
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);

    // Shadowing
    let y = 5;
    println!("{}", y);
    // this is allowed, even if not defined mut? wth?
    let y = y + 1;
    println!("{}", y);

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    // OK. The reason it works, is because using let before the variable. This is
    // called shadowing!
    // Shadowing is different from marking a variable as mut, because we'll get a 
    // compile-time error if we accidentally try to reassign to this variable 
    // without using the let keyword. By using let, we can perform a few transformatsion
    // on a value but have the variable be immutable after those transformations
    // have been completed.

    // but what happens if i try to print y in another scope?
    {
        {
            println!("{}", y);
        }
    }
    // 6! Of course it won't read the scope from the previous. It will take the last
    // global scope value.
    
    // Another difference from mut, is that we are effectively creating a new variable
    // with let. 
    let ken="1";
    let ken="2";
    let ken="3";
    println!("{}", ken);


    // This last example here is quite nice. If used with let mut spaces, then
    // spaces = spaces.len(); it would give a error on compile time, mismatch type.
    // maybe best example of differences between shadowing and let. 

    let spaces = "       ";
    let spaces = spaces.len();
    println!("{}", spaces);
}
