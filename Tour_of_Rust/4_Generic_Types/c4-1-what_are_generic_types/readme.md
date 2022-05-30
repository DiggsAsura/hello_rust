# What Are Generic Types?

Generic types allow us to partially define a struct or enum, enabling a compiler to create fully
defined version at compile-time based off our code usage.

Rust generally can infer the final type by looking at our instantiation, but if it needs help
you can always be explicit using the ::<T> operator, also known by the name *turbofish* (he's
a good friend of mine!).
