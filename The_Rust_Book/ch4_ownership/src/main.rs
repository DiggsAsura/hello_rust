// Starting the next big chapter in the rust-book
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

fn main() {
    println!("The Stack and the Heap");
    println!("------------------------\n");

    // STACK
    println!("The stack stores values in the order it gets them and removes the values in the opposite order.");
    println!("This is referred to as \"last in, first out\"");

    println!("STACK = Plates");
    println!("When you add more plates, you puth them on top of the pile, and when you need a plate, you take one off the top.");
    // good one!

    println!("\nAdding data is called pushing onto the stack, and removing data is called popping off the stack.");
    println!("All data stored on the stack must have a known, fixed size!");
    println!("Data with unkown seize at compile time or a size that might change must be stored on the heap instead.\n");

    // OK sooo, how do I tell rust to store on stack vs heap?
    

    // HEAP
    println!("The heap is less orgainized. When you place stuff on the heap, you request a certain amount of space,");
    println!("then the memory allocator finds an empty spot in the heap at the right size, marks it as being used, and");
    println!("returns a pointer - which is the address of that location.");
    println!("This process is called allocating on the heap, and is sometimes abbreviated as just allocating (pushing)");
    println!("values onto the stack is not considered allocating). Because the pointer to the heap is a known, fixed size, you can");
    println!("store the pointer on the stack, but when you want the actual data, you must follow the pointer");
    // Resturant
    println!("Thik of being seated at a resturant!");
    println!("When you enter, you state the number of people in your group, and the staff finds an empty table that fits everyone and leads you there.");
    println!("If someone in your group comes late, they can ask where you've been seated to find you.\n");
    // good stuff again.
    

    // HEAP VS STACK
    println!("Accessing data in the heap is slower than the stack because you have to follow a pointer to get there");
    println!("Contemporary (modern) processors are faster if they jump around less in memory.\n");

    // Sum it up
    println!("Summary:");
    println!("When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function's local variables get pushed onto the stack.");
    println!("When the function is over, those values get popped off the stack.\n");

    println!("Keeping track of what parts of code are using what data on the heap, minimizing the amount of");
    println!("duplicate data on the heap, and cleaning up unused data on the heap so you don't run out of");
    println!("space are all problems that ownership addresses. Once you understand ownership, you won't need to think");
    println!("about the stack and the heap very often, but knowing that the main purpose of ownership is to");
    println!("manage heap data can explain why it works the way it does");
}
