# Chapter 5 - Ownership & Borrowing Data

Rust has a unique paradigm for managing memory compared to other programming languages.
We're going to look at the behaviors and validations of the vompiler one by one so it's
not overwhelming. It's important to remember that ultimately the rules we show don't
exist to make your life hard, but to help you make your code less error-prone.


"not overwhelming", lol. 

Rip xD


---------------

# Chapter 5 - Conclusion

Whew, congrats for making it through (well it's all fake, i dont understand
shit still!)! I know it's a lot to take in, but you are well under way to becoming a Rustacean (lol somewhat cringe tbh).
Hopefully it's clear how Rust as a language aims to solve many of these
common challenges in systems programming:

* Unintentional modifications of resources
* Forgetting to deconstruct resources
* Resources accidentally being deconstructed twice
* Using resources after they have been deconstructed
* Data races caused by writing to resources while others are reading from resources
* Seeing clearly areas of the code where the compiler can't make guarantees

In the next chapter we'll apply some of this knowledge as we look at how
Rust handles text.


