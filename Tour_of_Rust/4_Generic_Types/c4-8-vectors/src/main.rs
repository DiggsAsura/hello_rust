fn main() {
  // We can be explicit with type
  let mut i32_vec = Vec::<i32>::new(); // turbofish <3
  i32_vec.push(1);
  i32_vec.push(2);
  i32_vec.push(3);

  // But look how clever Rust is about determining the type automatically
  let mut float_vec = Vec::new();
  float_vec.push(1.3);
  float_vec.push(2.3);
  float_vec.push(3.4);

  // That's a beautiful macro!
  let string_vec = vec![String::from("Hello"), String::from("World")];

  for word in string_vec.iter() {
    println!("{}", word);
  }
}
