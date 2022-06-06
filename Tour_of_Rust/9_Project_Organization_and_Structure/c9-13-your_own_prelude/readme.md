# Your Own Prelude

Because of standard library's prelude, it's common for your library to have its own
prelude module as a starting point for where users should import all of the most 
common data structures for using your library (e.g **use my_library::prelude::\***). 
It doesn't automatically get used in programs/libraries that use your crate but it's a
good convention to follow so people know where to start.

Ferris says, "Be a good rustacean and help fellow crab out with a good prelude!"
