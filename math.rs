
#[derive(Debug)]
enum Expr {
  Lit(i32),
  Var(String),
  Add(Box<Expr>, Box<Expr>),
  Mult(Box<Expr>, Box<Expr>),
}

use Expr::*;

fn simplify(e: &Expr) -> Expr {
  match *e {
    Lit(l) => Lit(l),
    Var(ref s) => Var(s.clone()),
    Add(ref l, ref r) => {
      let lhs = simplify(l);
      let rhs = simplify(r);
      match (lhs, rhs) {
        (Lit(l), Lit(r)) => Lit(l + r),
        (l, r) => Add(Box::new(l), Box::new(r))
      }
    },
    Mult(ref l, ref r) => {
      let lhs = simplify(l);
      let rhs = simplify(r);
      match (lhs, rhs) {
        (Lit(l), Lit(r)) => Lit(l * r),
        (l, r) => Mult(Box::new(l), Box::new(r))
      }
    }
  }
}

fn test_simplify(tree: Expr) {
  println!("simplify({:?}) = {:?}", tree, simplify(&tree));
}

fn main() {
  test_simplify(Add(
    Box::new(Mult(
      Box::new(Lit(4)),
      Box::new(Lit(5))
    )),
    Box::new(Var("x".to_string()))
  ));
}
