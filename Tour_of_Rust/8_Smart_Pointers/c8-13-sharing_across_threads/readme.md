# Sharing Across Threads

**Mutex** is a container data structure commonly held by smart pointers that takes
in data and lets us borrow mutable and immutable references to the data within. This
prevents borrowing from being abused by having the operating system restrict only one
CPU thread at time to have access to the data, blocking other threads until that original
thread is done with its locked borrow.

Multithreading is beyond scope of Tour of Rust, but **Mutex** is a fundamental part of
orchestrating multiple CPU threads accessing the same data.

There is a special smart pointer **Arc** which is identical to **Rc** except uses thread-safe
incrementing of reference counts. It's often used to have many references to the same
**Mutex**.


