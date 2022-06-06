# Referencing Other Modules and Crates

Items in modules can be referenced with their full module path
**std::f64::const::PI**.

A simpler way is the **use** keyword. It allows us to specify particular items from
modules we want to use throughout our code without a full path. For instance **use
std::f64::consts::PI** allows me to just use the identifier **PI** in my main function.

**std** is the crate of the **standard library** of Rust which is full of useful data
structures and functions for interacting with your operating system.

A searchable directory of crates created by the community can be found at 
https://crates.io


