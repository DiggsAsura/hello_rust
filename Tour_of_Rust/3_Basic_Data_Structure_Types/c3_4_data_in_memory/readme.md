# Creating Data In Memory

When we instantiate a struct in our code our program creates the associated
field data side by side in memory.

We instantiate by specifyint all field values within 
StructName { ... }

Struct fields are accessed using a dot operator .

Memory details of our example:

* The text inside the quotes is read only data (e. g. "Ferris"), therfore it is placed in the data memory region.

* The function call String::from creates a struct String that is placed side by side with the fields of SeaCreature in the *stack*. A String represents text that can be changed and does this by:

    1. Creating memory on the *heap* for the text where it can be modified
    2. Storing a refrence to that memory location on the *heap* and storing it in String struct (More on this in future lessons)

* Finally our two friends *Ferris* and *Sarah* have data structures that will always have fixed locations in our program, so they are placed on the *stack*.


OH, a little a-ha here. Could stucts be somewhat Rust's version of classes? 