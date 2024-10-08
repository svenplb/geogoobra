use crate::expr::Expr;

impl Expr {
    pub fn display(&self) -> String {
        match self {
            Expr::Const(val) => format!("{}", val),
            Expr::Var(name) => name.clone(),
            Expr::Add(left, right) => format!("({} + {})", left.display(), right.display()),
            Expr::Sub(left, right) => format!("({} - {})", left.display(), right.display()),
            Expr::Mul(left, right) => format!("({} * {})", left.display(), right.display()),
            Expr::Div(left, right) => format!("({} / {})", left.display(), right.display()),
        }
    }
}
