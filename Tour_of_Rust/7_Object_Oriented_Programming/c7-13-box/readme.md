# Box

**Box** is a data structure that allows us to move our data from the stack to the heap.

**Box** is a struct known as a smart pointer that holds the pointer to our data on the heap.

Because **Box** is a struct with a known size (because it just holds a pointer), it is 
often used as a way to store a reference to something in a struct that must know the size
of its fields.

**Box** is so common it can be used from anywhere:

Box::new(Foo {...})


