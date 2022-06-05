# The * Operator

The * (star/astrics) operator is an explicit way to dereference a reference.

let a: i32 = 42;
let ref\_ref\_ref\_a: &&&i32 = &&&a;
let ref\_a: &i32 = \**ref\_ref\_ref\_a;
let b: i32 = \*ref_a;

Memory detail:

* Because i32 is a primitive type that implements the **Copy** trait, the bytes of variable **a** on stack are copied into the bytes of variable **b**.

