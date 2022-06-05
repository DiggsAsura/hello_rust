# Familiar Friends

Consider some smart pointers we've already seen like Vec<T> and String.

Vec<T> is a smart pointer that just owns some memory region of bytes. The Rust
compiler has no idea what exists in these bytes. The smart pointer interprets what it means
to grab items from the region of memory it manages, keeps track of where data structures
within those bytes begin and end, and then finally dereferences a raw pointer into data
structures into a nice clean ergonomic interface for us to use(e.g. my\_vec[3]).

Similarly, String keeps track of a memory region of bytes, and programmatically restricts
content written to it to always be valid **utf-8** and helps dereference the memory
region into a type **&str**.

Both these datastructures use unsafe dereferencing of raw pointers to do their job.

Memory details:

* Rust has an equivalent of C's **malloc** using alloc and Layout for getting ahold of 
your own memory retions to manage.

https://doc.rust-lang.org/std/alloc/fn.alloc.html
https://doc.rust-lang.org/std/alloc/struct.Layout.html
