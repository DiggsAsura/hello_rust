# Encapsulation With Methods

Rust supports the concept of an *object* that is a struct associated with some functions
(also known as methods).

The first parameter of any method must be a reference to the instance associated
with the method call (e.g. **instance0f0bj.foo()**). Rust uses:

* **&self** - Immutable reference to the instance.
* **&mut self** - Mutable reference to the instance.

Methods are defined within an implementation block with keyword **impl**:

impl MyStruct {
    ...
    fn foo(&self) {
        ...
    }
}
