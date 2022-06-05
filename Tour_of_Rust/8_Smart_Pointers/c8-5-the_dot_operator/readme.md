# The . operator

The **.** operator is used in accessing fields and methods of a reference. It works a bit 
more subtly.

let f = Foo { value: 42 };  
let ref\_ref\_ref\_ref\_f = &&&f;  
println!("{}", ref\_ref\_ref\_ref\_f.value);

Whola, why didn't we need to add triple astrics before ref\_ref\_ref\_ref\_f? This
is because the . operator automatically dereference a sequence of references. That last line
is turned into the following by the compiler automatically.

println!("{}", (***ref_ref_ref_ref_f).value);
