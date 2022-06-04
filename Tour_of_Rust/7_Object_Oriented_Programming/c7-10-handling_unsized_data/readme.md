# Handling Unsized Data

Traits introduce an interesting challenge when we want to store them within another struct. 
Traits obfuscate the original size. Unsized values being stored in structs are handled
in two ways in Rust:

* **generics** - using parameterized types effectively create struct/functions known types and thus known sizes.
* **indirection** - Putting instances on the heap gives us a level of indirection that allow us to not have to worry about the size of the actual type and just store a pointer to it. There are other ways as well!
