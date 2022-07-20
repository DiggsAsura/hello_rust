// generate the nth Fibonacci number
use std::io;

fn main() {
  let mut n = String::new();
  io::stdin()
      .read_line(&mut n)
      .expect("Failed");
  let n = n.trim().parse().unwrap();


  let (mut a, mut b)=(0, 1);

      for _ in 2..n {
          (a, b)=(b, a+b);
      }

  println!("{b}");
}

