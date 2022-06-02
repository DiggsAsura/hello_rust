fn main() -> Result<(), std::num::ParseIntError> {
  let a = 42;
  let a_string = a.to_string();
  let b = a_string.parse::<i32>()?;
  println!("{} {}", a, b);
  Ok(())
}

// I really just have to take a moment and LOL ROFL LMAO. This is so dam fucking hard
// to understand and make sense of. Theres like moments where things allmost makes sense,
// then suddenly its going all mulholland drive and nothing does anymore.
// god dam sarcastic stuff. 
