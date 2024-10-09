use std::collections::HashMap;

mod expr;

use expr::parser::{parse_expression, tokenize};
use expr::Expr;

fn main() {
    let expr = Expr::Add(
        Box::new(Expr::Const(2.0)),
        Box::new(Expr::Mul(
            Box::new(Expr::Var("x".to_string())),
            Box::new(Expr::Const(3.0)),
        )),
    );
    println!("{}", expr.display());
}
