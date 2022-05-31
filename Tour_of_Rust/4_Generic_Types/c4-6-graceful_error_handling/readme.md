# Graceful Error Handling

Result is so common that Rust has a powerful operator ? for working with them. These two statements
are equivalent:

do_something_that_might_fail()?

match do_something_that_might_fail() {
    Ok(v) => v,
    Err(e) => return Err(e),
}

