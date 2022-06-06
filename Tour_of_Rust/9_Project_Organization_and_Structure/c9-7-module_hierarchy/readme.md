# Module Hierrarchy

A module can depend on another one. in order to establish a relationship between a module
and its sub-module, you must write in the parent module:

**mod foo;**

The declaration above will look for a file named **foo.rs** or **foo/mod.rs** and will
insert its contents inside a module named **foo** under this scope.


No code example.
