# Smart Pointers

In addition to the ability to create references to existing typed data using the **&** operator,
Rust gives us the ability to create *reference-like* structs called **smart pointers**.

We can think of references at a high level as a type that give us access to another type. Smart 
pointers are different in their behavior from normal references in that they operate based on
internal logic that a programmer writes. You - the programmer - are the *smart* part.

Typically smart pointers implement **Deref, DerefMut** and **Drop** traits to specify the logic
of what should happen when the structure is dereferenced with * and . operators.



