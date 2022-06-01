struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f is dropped here
}

fn main() {
    let foo = Foo { x: 42 };
    // foo is moved to do_something
    do_something(foo);
    // foo can no longer be used
}
