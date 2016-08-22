
fn main() {
  let total:i32 = (1..1001).filter(|n| n % 3 == 0 && n % 5 == 0).sum();
  println!("Total is {}", total);
}
