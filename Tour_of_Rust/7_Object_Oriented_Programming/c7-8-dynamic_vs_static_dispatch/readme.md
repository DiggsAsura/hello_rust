# Dynamic vs Static Dispatch

Methods are executed in two ways:

* static dispatch - When the instance type is known, we have direct knowledge of what function to call
* dynamic dispatch - When an instance type is not known, we must find out some way of calling the correct function

Traits types &dyn MyTrait give us the ability to work with instances of objects
indirectly using dynamic dispatch.

When dynamic dispatch is used, Rust will encourage you to puh **dyn** before your
trait type so people are aware.

Memory details:

* Dynamic dispatch is slightly slower because of the pointer chasing to find the real function call.
