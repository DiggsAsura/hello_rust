struct Foo {
  x: i32,
}

fn main() {
  let foo_a = Foo { x: 42 };
  let foo_b = Foo { x: 13 };

  println!("{}", foo_a.x);
  println!("{}", foo_b.x);
  // foo_b is dropped here
  // foo_a is dropped here
  // ???? 
}
