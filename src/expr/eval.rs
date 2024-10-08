use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Expr {
    Const(f64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval(&self, vars: &HashMap<String, f64>) -> Result<f64, String> {
        match self {
            Expr::Const(val) => Ok(*val),
            Expr::Var(name) => vars.get(name).cloned().ok_or_else(|| format!("Unknown variable: {}", name)),
            Expr::Add(left, right) => {
                let left_val = left.eval(vars)?;
                let right_val = right.eval(vars)?;
                Ok(left_val + right_val)
            },
            Expr::Sub(left, right) => {
                let left_val = left.eval(vars)?;
                let right_val = right.eval(vars)?;
                Ok(left_val - right_val)
            },
            Expr::Mul(left, right) => {
                let left_val = left.eval(vars)?;
                let right_val = right.eval(vars)?;
                Ok(left_val * right_val)
            },
            Expr::Div(left, right) => {
                let left_val = left.eval(vars)?;
                let right_val = right.eval(vars)?;
                Ok(left_val / right_val)
            }
        }
    }
}
