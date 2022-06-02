# String

A **String** is a struct that owns a sequence of utf-8 bytes in heap memory.

Because its memory is on the heap, it can be extended, modified, etc. in ways string 
literals cannot. 

Common methods:

* **push_str** to add more utf-8 bytes to the end of a string.
* **replace** to replace sequences of utf-8 bytes with others.
* **to_lowercase** / **to_uppercase** for case changes.
* **trim** for trimming space.

When a String is dropped, its heap memory is also dropped.

**String** has a **+** operator that extends the string with a **&str** and returns
itself, but it might not be as ergonomic as you hope for.
