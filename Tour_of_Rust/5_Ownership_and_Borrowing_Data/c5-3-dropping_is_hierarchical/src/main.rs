struct Bar {
  x: i32,
}

struct Foo {
  bar: Bar,
}

fn main() {
  let foo = Foo { bar: Bar { x: 42 } };
  println!("{}", foo.bar.x);
  // foo is dropped first
  // then foo.bar is dropped
  // I DONT HAVE A FUCKING IDEA WHATS HAPPENING
  // lol
  // this gonna be a long run
}
