
fn main() {
  let limit:i64 = 4_000_000;
  let mut prev:i64 = 0;
  let mut curr:i64 = 1;

  let mut sum:i64 = 0;
  while curr < limit {
    if curr % 2 == 0 {
      sum += curr;
    }
    let temp_curr = curr;
    curr = prev + curr;
    prev = temp_curr;
  }
  println!("Sum {}", sum);
}
