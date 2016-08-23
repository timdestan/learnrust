
fn main() {
  let limit:i32 = 1000;
  let total:i32 = (1..limit).filter(|n| n % 3 == 0 || n % 5 == 0).sum();
  println!("Total is {}", total);
}
