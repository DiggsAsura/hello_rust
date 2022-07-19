// Chapter 3.5 - Control Flow
// ============================
// if expressions and loops


// if expressions 
// ----------------
fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    // if not providing else, and the condition is false, the program will just skip
    // the if block/arm and move to the next bit of code
    } else {
        println!("Condition was false");
    }

    // it's also worth noting that the condition in this code MUST be a bool. If the condition
    // isn't a bool, we'll get an error. For example, try running the following code:
    //if number {
    //    println!("number was {number}");
    //}

    // Nope, above won't run. 
    // The error indicates that Rust expected a bool but got an integer. Unlike languages
    // such and Ruby and JavaScript, Rust will not automatically try to convert non-Boolean
    // types to a Boolean. You must be explicit and always provide if with a Boolean as
    // its condition. If we want the if code block to run only when a number is not equal
    // to 0, for example, we can change the if expression to the following:

    if number != 0 {
        println!("number was something other than zero");
    }

    // 123 Following parts
    part_two(12);
    part_three();
    part_four();
    part_five();
    part_six();
    part_seven();
    part_eight_while();
    part_eight_for();
    part_eight_for_range();
}


// Part 2: Handling Multiple Conditions with else if
// --------------------------------------------------
// example handle if/else if/else

fn part_two(number: i32) {
    //let number: i32 = 6;
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0{
        println!("number is divisible by 2");
    } else {
        println!("number is not devisible by 4, 3 or 2");
    }
}
// Using too many if/else can clutter the code. We'll look at match later for that. 


// Part 3: Using if in a let Statement
// ----------------------------------------
// Because if is an expression, we can use it on the right side of a let statement to 
// assign the outcome to a variable:

fn part_three() {
    let condition = true; // 5, false becoming 6
    let number = if condition { 5 } else { 6 };

    println!("The value ofnumber is: {number}");
}
// let number = if condition { 5 } else { six }
// This will fail, because variables need one type only. Means both arms need to be same type
// or it will fail at compile time. 



// Part 4: Repeating Code with loop
// -----------------------------------

// The loop keyword tells Rust to execute a block of code over and over again forever
// until you explicitly tell it to stop. 
//
// As an example, try this. Ctrl-c to stop:

fn part_four() {
    let mut count = 0;
    loop {
        if count < 10 {
            println!("Count: {count}");
            count += 1;
            continue
        } else {
            break
        } 
    }
}



// Part 5: Returning Values from Loops
// --------------------------------------

fn part_five() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break AND return count * 2, well make it the expression of the
                               // let statement 
        }
    };

    println!("The result is {result}");
}

// Before the loop, we declare a variable named counter and initialize it to 0. Then we
// declare a variable named result to hold the value returned from the loop. On every iteration
// of the loop, we add 1 and the counter variable, and then check wheter the counter is 
// equal to 10. When it is, we use the break keyword with the value counter * 2. 
// After the loop, we use a semicolon to end the statement that assigns the value to 
// result. Finally, we print the value in result, which in this case is 20.


// Part 6: Loop Labels to Disambiguate Between Multiple Loops 
// --------------------------------------------------------------
//
// Loop labels need to start with a single quotation

fn part_six() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// The outer loop has the label 'counting_up and will count up from 0 to 2. The inner loop 
// without a label counts down from 10 to 9. The first break that doesn't specify a label
// will exit the inner loop only. The break 'countin_up; statement will exit the outer
// loop. 


// Part 7: Conditional Loops with while
// --------------------------------------
//
// loop, if, else - so common that you can rather use while. Piss poor summary, but it's
// just a god dam while loop lol.

fn part_seven() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// This construct (while) eliminates a lot of nesting that would be necessary if you used
// loop, if, else, and break, and it's clearer. While a conditions hold true, the code runs,
// otherwise, it exits the loop. 



// Part 8: Looping Through a Collection with for
// -----------------------------------------------
//
// I guess, the for loop pretty much? Eh? 

// loop through an array with while, to see how much better it is with for lol

fn part_eight_while() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }
}

// Here, the code counts up through the elements in the array. It start at index 0, and
// then loops until it reaches the final index in the array (that is, when index < 5 is
// no longer TRUE). Running this code will print every element in the array. 

// This is not good, it is prone to error and can panic if f.x not enough items in the
// array. It's also slow, because the compiler adds runtime code to perform the conditional
// check of wheter the index is within the bounds of the array on every iteration 
// of the loop. 

// A better way is ofc the for loop:

fn part_eight_for() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}

// the for loop is so much used in rust, we usually use it even where while might be 
// usually applied..? did i get that right? 

// countdown example with for and range (note the range, i like that in rust)
fn part_eight_for_range() {
    for num in (1..4).rev() {
        println!("{num}!");
    }
    println!("LIFTOFF!!!");
}

// This code is a bit nicer, isnt't it? 
//
// ...hmm is .rev() function just reverse? Looks like it.



// Summary
// --------
//
// You made it! That was a sizeable chapter: you learned about variables, scalar and compound
// data types, functions, comments, if expressions, and loops! To practice with the concepts
// discussed in this chapter, try building programs to do the following:
//
// Convert temperatures between Farenheit and Celcius
// Generate the nth Fibonacci number
// Print the lyrics to the Christmas carol "The Twelve Days of Christmas", taking 
//      advantage of the repetition in the song
//
// When your're ready to move on, we'll talk about a concept in Rust that doesn't commonly
// exist in other programming languages: ownership.
