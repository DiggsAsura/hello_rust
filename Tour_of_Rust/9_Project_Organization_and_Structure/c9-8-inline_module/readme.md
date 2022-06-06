# Inline Module

A sub-module can be directly inlined within a module's code.

One very common use for inline modules is creating unit tests. We create an inline
module that only exists when Rust is used for testing!


// this macro removes this inline module when Rust
// is not in test mode.

\#[cfg(test)]
mod tests {
    // Notice that we don't immediately get access to the 
    // parent module. We must be explicit.
    use super::*;

    ... tests go here ...
}



No code example that runs
