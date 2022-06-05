# Failable Main Revisited

Rust code may have a plethora of representations of erros, but the standard library
has a universal trait **std::error::Error** for describing errors.

Using a smart pointer **Box** we can use the type **Box<dyn std::error::Error>** as
a common type for returning errors because it allows us to propagate up an error on
the heap and interact with it at a high level without having to know a specific type.

Eary in Tour of Rust we learend that **main** can return an error. We can now return a
type capable of describing almost any kind of error that might occur in our program so
long as the error's data strucutre implements Rust's common **Error** trait.

**fn main() -> Result<(), Box<dyn std::error::Error>>

