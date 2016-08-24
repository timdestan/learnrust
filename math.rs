
#[derive(Debug)]
enum Expr {
  Lit(i32),
  Add(Box<Expr>, Box<Expr>),
  Mult(Box<Expr>, Box<Expr>),
}

use Expr::*;

fn simplify(e: &Expr) -> i32 {
  match *e {
    Lit(l) => l,
    Add(ref l, ref r) => simplify(&*l) + simplify(&*r),
    Mult(ref l, ref r) => simplify(&*l) * simplify(&*r)
  }
}

fn main() {
  let tree = Add(
    Box::new(Mult(
      Box::new(Lit(4)),
      Box::new(Lit(5))
    )),
    Box::new(Lit(7))
  );
  println!("simplify({:?}) = {}", tree, simplify(&tree));
}
