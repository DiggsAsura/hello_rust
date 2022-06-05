# Raw Pointers

References can be converted into a more primitive type called *raw pointer*. Much like a
number, it can be copied and moved around with little restrictions. Rust makes no assurances of 
the validity of the memory locations it points to.

Two kinds of raw pointers exist:

* const T - A raw pointer to data of type T that should never change.
* mut T - A raw pointer to data of type T that can change. 

Raw pointers can be converted to and from numbers (e.g. **usize**).

Raw pointers can access data with *unsafe* code (more on this later).

Memory Details:

* A reference in Rust is very similar to a pointer in C in terms of usage, but with much more compile time restrictions on how it can be stored and moved around to other functions.
* A raw pointer in Rust is similar to a pointer in C that it represents a number that can be copied or passed around, and even turned into numerical types where it can be modified as a number to do pointer math.
