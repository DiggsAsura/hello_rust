/* 
Structures

A struct is a collections of fields.

A field is simply a data value associated with a data structure. Its value can be of a 
primitive type or a data structure.

Its definition is like a blueprint for a compiler on how to layout the fields in
memory nearby each other.

*/

struct SeaCreature { 
    // String is a struct
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}