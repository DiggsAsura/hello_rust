# String Literals

String literals are always Unicode.

String literals type are &'static str:

* & meaning that it's referring to a place in memory, and it lacks a &mut meaning that the compiler will not allow modifications
* 'static meaning the string data will be available till the end of our program (it never drops)
* str means that it points to a sequence of bytes that are always valid **utf-8**

Memory details:
* The Rust compiler will likely put your string in the data segment of your program memory


