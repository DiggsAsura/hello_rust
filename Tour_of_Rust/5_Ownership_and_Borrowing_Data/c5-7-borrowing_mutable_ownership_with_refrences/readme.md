# Borrowing Mutable Ownership with References

We can also borrow mutable access to a resource with the &mut operator.

A resource owner cannot be moved or modified while mutably borrowed.

Memory detail:
* Rust prevents having two ways to mutate an owned value because it introduces the
possibility of a data race

