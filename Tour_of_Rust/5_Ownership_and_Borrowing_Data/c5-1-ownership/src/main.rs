struct Foo {
  x: i32,
}

fn main() {
  // We instantiate structs and bind to variables
  // to create memory resources
  let foo = Foo { x: 42 };
  // foo is the owner
  //println!("{:?}", foo) // wanted to test, but could not test as the example itself fails lol. Rust is hard
}
