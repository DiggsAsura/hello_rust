fn main() {
    // Data Types
    let guess: u32 = "42".parse().expect("Not a number!");
    let sum = guess + guess;
    println!("{guess} + {guess} = {sum}");

    // Number Literals
    let decimal = 98_222; // _ will still be 98222 
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A'; // u8 only

    println!("Decimal: {decimal}");
    println!("Hex: {hex}");
    println!("Octal: {octal}");
    println!("Binary: {binary}");
    println!("Byte (u8 only): {byte}");

    
    // Floating-Point Types
    let float_1_32: f32 = 0.1;
    let float_2_32: f32 = 0.2;
    let float_3_32: f32 = float_1_32 + float_2_32;

    let float_1_64: f64 = 0.1; // f64 default
    let float_2_64 = 0.2;
    let float_3_64 = float_1_64 + float_2_64;

    println!("Float 32: {float_1_32} + {float_2_32} = {float_3_32}");
    println!("Float 64: {float_1_64} + {float_2_64} = {float_3_64}");

    
    // The Character Type
    // Rust's char type is the language's most primitive alphabetic type. Here's some 
    // examples of declaring char values:

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c: {c}");
    println!("z: {z}");
    println!("Emoji: {heart_eyed_cat}");


    // Compound Types
    // ===============
    // Comput types can group multiple values into one type. Rust has two primitive 
    // compound types: tuples and arrays.
    //

    // The Tuple Type
    // ----------------
    // Tuples can hold a varity of types in one compound type. Fixed length, can not
    // append or remove. 

    let tup_a: (i32, f64, u8) = (500, 6.4, 1);
    // tup_a binds to the entire tuple, because tuple is considered a single compound
    // element. To get the individual values out of a tuple, we can use pattern matching
    // to destructure a tuple value, like this:

    let (x, y, z) = tup_a;
    println!("The value of y is {y}, z: {z}, x: {x}");
    // We not create a tuple and bind it to the variable tup_a. It then uses pattern 
    // with let to take tup_a and turn it into three separate variables, x, y, z. This
    // is called "destructuring", because it breaks the single tuple into three parts.
    
    // We can also access tuple elements directly by using a period . followed by the index
    // of the value we want to access:
    let x: (i32, f64, u8) = (600, 44.44, 123);
    let sixhundred = x.0;
    let secondvalue = x.1;
    let thirdvalue = x.2;
    println!("{sixhundred}, {secondvalue}, {thirdvalue}");

    // The tuple without any values has a special name, unit. This value and its corresponding
    // type are both written () and represent an empty value or an empty return type. 
    // Expressions implicitly return the unit value if they don't return any other value.

    // The Array Type
    // ----------------
    // Arrays in Rust have fixed length. I guess vec! for the python version of arrays/list.

    let a = [1, 2, 3, 4, 5];
    let three = a[2];
    println!("{three}");

    // Arrays are powerful when you want your data allocated on the stack rather than the
    // heap (we will discuss the stack and the heap more in Chapter 4) or when you want to 
    // ensure you always have a fixed number of elements. An array isn't as flexible as 
    // the vector type, though. A vector is similar collection type provided by the standard
    // library that IS allowed to grow or shrink in size. If you're unsure wheter to use
    // an array or a vector, chances are you should use a vector. Chapter 8 discuss vectors
    // more in detail.
    //
    // HOWEVER, arrays are more useful when you know the number of elements will not need
    // to change. For example, if you were using the names of the month in a program, you
    // would probably use an array rather than a vector because you know it will always
    // contain 12 elements: 

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    for month in months {
        println!("{month} is just the best month, no?");
    }

    // You write an array's type using square with the type of each element, a semicolon,
    // and then a nuber of elements in the array, like so:
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    // A fancy, consise way of adding a list of five 3's:
    let a = [3; 5]; // see the 3; 
    println!("{:?}", a);


    // Invalid Array Element Access
    // ------------------------------
    // This is a long example to prove panic when out of range. 

    use std::io;

    let aa = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = aa[index];

    println!("The value of the element at index {index} is: {element}");
}
