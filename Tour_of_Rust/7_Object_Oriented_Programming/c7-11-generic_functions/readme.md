# Generic Functions

Generics in Rust work hand in hand with traits. When we describe a parmaeterized type **T** 
we can constrain what types can be used as an argument by listing what required traits the
argument must implement. 

In this example *T* must implement trait **Foo**:

fn my_function<T>(foo: T)
where
    T:Foo
{
    ...
}


By using generics we create static typed functions at compile time that will have known types
and sizes, allowing us to perform static dispatch and store as a sized value.
