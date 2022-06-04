# Generic Structs Revisited

Generic structs can also have their parameterized types constrained by traits. 

struct MyStruct<T>
where
    T: MyTrait
{
    foo: T
    ...
}

Generic structs have their parameterized type in their implementation blocks:

impl<T> MyStruct<T> {
    ...
}


No code example.
No meaning given.
I'm lost AF. 
