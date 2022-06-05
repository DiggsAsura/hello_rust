# Smart Unsafe Code

Smart pointers tend to use unsafe code fairily often. As mentioned earlier, they are common
tools for interacting with the lowest levels of memory in Rust.

What is unsafe code? Unsafe code behaves exactly like normal Rust with the exception of 
a few abilities that the Rust compiler is unable to make guarntees about. 

A primary ability of unsafe code is *dereferencing a raw pointer*. That means taking 
a *raw pointer* to a position in memroy and declaring "a data structure exists here!"
and turning it into a representation of data you can use (i.e. \*const u8 into u8). Rust 
has no way to keep track of the meaning of every byte that gets written to memory.
Because Rust can't make guarantees about what exists at an arbitrary number used as a 
*raw pointer*, it puts the dereference in an **unsafe { ... }** block. 

Smart pointers *dereference raw pointers* extensively, but they are well proven in what 
they do.

