# Result

Rust has a built in generic enum called Result that allows us to return a value that has
the possibilty of failing. It is the idiomatic way in which the language does error handling.

enum Result<T, E> {
    Ok(T),
    Err(E),
}

Note that our generics type has multiple *parameterized types* separated by a comma.

This enum is so common, instances of the enum can be created anywhere with the enum 
variants Ok and Err.
