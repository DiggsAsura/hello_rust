# Dropping is Hierarchical

When a struct is dropped, the struct itself is dropped first, then its children are
dropped individually, and so on.

Memory Details:

* By automatically freeing memory Rust helps ensure that there are fewer memory leaks.
* Memory resource can only be dropped once.
