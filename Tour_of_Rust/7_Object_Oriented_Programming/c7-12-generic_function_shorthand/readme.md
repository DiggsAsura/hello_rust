# Generic Function Shorthand

Rust has a shorthand for expressing generics constrained by a trait:

fn my_function(foo: impl Foo) {
    ...
}

This is equivealent to writing:

fn my_function<T>(foo: T)
where
    T:Foo
{
    ...
}


