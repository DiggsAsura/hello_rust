# Referencing Counting

**Rc** is a smart pointer that moves data from the stack onto the heap. It allows us to 
clone other **Rc** smart pointers that all have the ability to immutably borrow the data
that was put on the heap.

Only when the last smart pointer is dropped does the data on the heap become deallocated.
