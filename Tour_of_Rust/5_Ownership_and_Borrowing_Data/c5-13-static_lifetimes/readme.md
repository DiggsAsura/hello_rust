# Static Lifetimes

A **static** variable is a memory resource created at compile-time that exists through a program start to finish. They must have their types explicitly specified. 

A **static lifetime** is a memory resource that lasts indefinetly to the end of a program.

Resources with static lifetimes have a special lifetime specifier 'static.

'static resources will never **drop**.

If static lifetime resources contain references they must all be 'static (anything less would not live long enough).

Memory detail:

* Modifying static variables is inherently dangerous because they are globally accessable to be read from by anyone introducing the possibility of a data race. We'll talk about the challenges of global data later.

* Rust allows the use of unsafe { ... } blocks to perform some operations that the compiler cannot make memory guarnatees about. The Rustonomicon should not be talked about casually.

