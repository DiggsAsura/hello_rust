// I'm a bit low on motivation and low on creativity. I have learned alot over the last 6 months but putting it all together is still not
// quite possible. I think! I do still very much enjoy though :) So some day ;)

fn main() {
  let my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
  
  let mut multiplied = 1;

  for i in my_vec {
        multiplied *= i;
        println!("{}", multiplied);
    }
}
