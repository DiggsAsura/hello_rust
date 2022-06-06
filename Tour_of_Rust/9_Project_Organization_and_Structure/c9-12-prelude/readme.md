# Prelude

You might be wondering how we have access to **Vec** or **Box** everywhere without a 
**use** to import them. It is becaue of the module **prelude** in the standard library.

Know that in the Rust standard library anything that is exported in **std::prelude::\***
is automatically available to every part of Rust. That is the case for **Vec** and **Box**
but others as well (Option, Copy, etc).
