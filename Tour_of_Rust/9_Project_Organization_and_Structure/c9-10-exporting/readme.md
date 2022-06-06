# Exporting

By default members of a *module* are not accessible from outside of the module (not 
even its child modules!). We make members of a module accessible using the **pub** keyword.

By default members of a *crate* are not accessible outside of the crate. We make members 
of a crate accessible by marking them as **pub** in the *root module* of your crate
(**lib.rs** or **main.rs**).
