# Explicit Lifetimes

Even though Rust doesn't always show it in code, the compiler understands the lifetime
of every variable and will attempt to validate that a reference never exists longer 
than its owner. 

Functions can be explicit by parameterizing the function signature with symbols that
help identify which parameters and return values share the same lifetime.

Lifetime specifiers always start with a ' (e.g. 'a, 'b, 'c)

