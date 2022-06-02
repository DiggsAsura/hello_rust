# Lifetimes in Data Types

Similarly to functions, data types can be parameterized with lifetime 
specifiers of its members.

Rust validates that the containing data structure of the references never
lasts longer than the owners its references point to.

We can't have structs running around with references pointing to 
nothingness!
