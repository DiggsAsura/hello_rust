# References Revisited

A reference is fundamentally just a number that is the start position of some bytes in 
memory. It's only purpose is to represent the concept of where data of a specific type exists.
What makes a reference different from just a number is that Rust will validate the lifetime
of references doesn't last longer than what it refers to (otherwise we'd get an error when we
used it!).

No code example
