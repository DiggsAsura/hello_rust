struct Foo {
  x: i32,
}

// the parameter foo and return value share the same lifetime
fn do_something<'a>(foo: &'a Foo) -> &'a i32 {
  return &foo.x;
}

fn main() {
  let mut foo = Foo { x: 42 };
  let x = &mut foo.x;
  *x = 13;
  // x is dropped here, allowing us to create a non-mutable reference
  let y = do_something(&foo);
  println!("{}", y);
  // y is dropped here
  // foo is dropped here
}
