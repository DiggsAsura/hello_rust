# String Slice

A String slice is a reference to a sequence of bytes in memory that must always be valid
utf-8.

A string slice (a sub-slice) of a str slice, must also be valid utf-8.

Common methods of &str:

* len gets the length of the string literal in bytes (not number of characters).
* starts_with / ends_with for basic testing.
* is_empty returns true if zero length.
* find returns an Option<usize> of the first position of some text.
