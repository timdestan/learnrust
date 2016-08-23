
// Find pythagorean triple a ^ 2 + b ^ 2 = c ^ 2 s.t. a + b + c = 1000.
// Output a * b * c
fn main() {
  'outer: for c in 1..1001 {
    for b in 1..(1001 - c) {
      let a = 1000 - b - c;
      if a * a + b * b == c * c {
        println!("{},{},{},{}", a, b, c, a*b*c);
        break 'outer;
      }
    }
  }
}
