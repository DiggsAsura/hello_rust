# Combining Smart Pointers

Smart pointers might seem limited, but they can make some very powerful combinations.

**Rc<Vec<Foo>>** - Allow the cloning of multiple smart pointers that can borrow the
same vector of immutable data structures on the heap.

**Rc<RefCell<Foo>>** - Allow multiple smart pointers the ability to borrow mutably/immutably the same struct **Foo**

**Arc<Mutex<Foo>>** - Allow multiple smart pointers the ability to lock temporary mutable/immutable borrows in a CPU thread exclusive manner.

Memory detail:

* You'll notice a theme with many of these combinations. The use of a immutable data type
(possibley owned by multiple smart pointers) to modify internal data. This is referred
to as the "interior mutability" pattern in Rust. It is a pattern that lets us bend
the rules of memory usage at runtime with the same level of safety as Rust's compile-time
checks.
