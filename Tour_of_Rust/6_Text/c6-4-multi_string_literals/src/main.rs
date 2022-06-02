fn main() {
  let haiku: &'static str = "
    I write, erase, rewrite
    Erase again, and then
    A poppy blooms.
    - Katsushika Hokusai";
  println!("{}", haiku);

  println!("hello \
  world") // notice that the spacing before w is ignored
}
