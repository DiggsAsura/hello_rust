# Passing Around Borrowed Data

Rust's rules for references might best be summarized by:

* Rust only allows there to be one mutable reference **or** multiple non-mutable references **but not both**.
* A reference must never **live longer** than its owner.

This doesn't tend to be a problem when passing around references to functions.

Memory details:

* The first rule of references prevents data races. What's a data race? A data race when reading 
  from data has the possibility of being out of sync due to the existence of a writer to the data at 
  at the same time. This happens often in multi-threaded programming.

* The second rule of refrences prevents the maisuse of references that refer to 
  non-existent data (called dangling pointers in C).

