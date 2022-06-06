# Sharing Access

**RefCell** is a container data structure commonly held by smart pointers that takes in
data and lets us borrow mutable and immutable references to what's inside. It prevents
borrowing from being abused by enforcing Rust's memory safety rules at runtime when 
you ask to borrow the data within:

**Only one mutable reference OR multiple immutable references, but not both!**

If you violate these rules **RefCell** will panic.
