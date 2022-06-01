# Moving Ownership

When an owner is passed as an argument to a function, ownership is moved tothe function parameter.

After a **move** the variable in the original function can no longer be 
used.

Memory in details:

* During a **move** the stack memory of the owners value is copied to the
function call's parameter stack memory.

