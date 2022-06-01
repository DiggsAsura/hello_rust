struct Foo {
  x: i32,
}

fn do_something(f: &mut Foo) {
  f.x += 1;
  // mutable refrence f is dropped here
}

fn main() {
  let mut foo = Foo { x: 42 };
  do_something(&mut foo);
  // because all mutable refrences are dropped within
  // the function do_something, we can create another.
  do_something(&mut foo);
  // foo is dropped here
}
