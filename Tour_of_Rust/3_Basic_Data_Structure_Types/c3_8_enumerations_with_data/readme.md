# Enumerations With Data

enum elements can also have one or more data types allowing them to behave like *union* from C (lol right, idk wtf that is).

When an enum is pattern matched using match, you can bind a variable name to each data value.

Memory details of enum:

1. An enum data value will have a memory size equal to its largest element. This allows for all potential values to fit in the same space of memory.

2. In addition to element data types (if any), each element also has a numeric value that represents which tag it is. 

Other details:

* Rust's enum is something also known as a *tagged union*.
* The combining of types to make a new type is what people mean when they say Rust has algebraic types
