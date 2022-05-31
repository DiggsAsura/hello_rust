# Vectors

Some of the most useful generic types are collection types. A vector is a variable 
sized list of items represented by the struct Vec.

The macro vec! lets us easily create a vector rather than manually constructing one.

Vec has the method iter() which creates an iterator from a vector, allowing us to
easily put a vector into a for loop.

Memory Details:

* Vec is a struct, but internally contains a refrence to a fixed list of its items on the heap.
* A vector starts with a default capacity; when more items are added than it has capacity for, it reallocates its data on the heap to have a new fixed list with large capacity.

