fn main() {
  let a = "hi crab";
  println!("{}", a.len());
  let first_word = &a[0..2];
  let second_word = &a[3..7];
  // let half_crab = &a[3..5]; FAILS
  // Rust does not accept slices of invalid unicode characters
  println!("{} {}", first_word, second_word);

}
