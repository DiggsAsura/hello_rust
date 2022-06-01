fn main() {
  let mut foo = 42;
  let f = &mut foo;
  let bar = *f; // get a copy of the owner's value
  *f = 13;      // set the refrence's owners value 
  println!("{}", bar);
  println!("{}", foo);
}
