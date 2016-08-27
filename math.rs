
#[derive(Debug)]
enum Expr {
  Lit(i32),
  Var(String),
  Add(Box<Expr>, Box<Expr>),
  Mult(Box<Expr>, Box<Expr>),
}

use Expr::*;

fn maybe_simplify_literals<F, G>(
    lhs: Expr,
    rhs: Expr,
    combine_literals: F,
    rebuild: G) -> Expr
    where F : Fn(i32, i32) -> Expr,
          G : Fn(Box<Expr>, Box<Expr>) -> Expr {
  match (lhs, rhs) {
    (Lit(lv), Lit(rv)) => combine_literals(lv, rv),
    (le, re) => rebuild(Box::new(le), Box::new(re))
  }
}

fn simplify(e: &Expr) -> Expr {
  match *e {
    Lit(l) => Lit(l),
    Var(ref s) => Var(s.clone()),
    Add(ref l, ref r) => {
      maybe_simplify_literals(simplify(l), simplify(r),
                              |l, r| Lit(l + r),
                              |l, r| Add(l, r))
    },
    Mult(ref l, ref r) => {
      maybe_simplify_literals(simplify(l), simplify(r),
                              |l, r| Lit(l * r),
                              |l, r| Mult(l, r))
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
  test_simplify(Add(
    Box::new(Mult(
      Box::new(Lit(3)),
      Box::new(Lit(12))
    )),
    Box::new(Lit(14))
  ));
  test_simplify(Add(
    Box::new(Mult(
      Box::new(Var("y".to_string())),
      Box::new(Var("z".to_string()))
    )),
    Box::new(Var("x".to_string()))
  ));
}
