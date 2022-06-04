# Trait Objects

When we pass an instance of an object to a parameter of type &dyn MyTrait we pass what is 
called a trait object.

A trait object is what allows us to indirectly call the correct methods of an instance. A trait
object is a struct that holds the pointer of our instance with a list of function pointers to our
instance's methods.

Memory details:

* This list of functions is known in C++ as a vtable.

