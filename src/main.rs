use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Expr {
    Const(f64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn display(&self) -> String {
        match self {
            Expr::Const(val) => format!("{}", val),
            Expr::Var(name) => name.clone(),
            Expr::Add(left, right) => format!("({} + {})", left.display(), right.display()),
            Expr::Sub(left, right) => format!("({} - {})", left.display(), right.display()),
            Expr::Mul(left, right) => format!("({} * {})", left.display(), right.display()),
            Expr::Div(left, right) => format!("({} / {})", left.display(), right.display()),
        }
    }

    fn eval(&self, vars: &HashMap<String, f64>) -> Result<f64, String> {
        match self {
            Expr::Const(val) => Ok(*val),
            Expr::Var(name) => {
                vars.get(name)
                    .cloned()
                    .ok_or_else(|| format!("Unknown variable: {}", name))
            }
            Expr::Add(left, right) => {
                let left_val = left.eval(vars)?;
                let right_val = right.eval(vars)?;
                Ok(left_val + right_val)
            }
            Expr::Sub(left, right) => {
                let left_val = left.eval(vars)?;
                let right_val = right.eval(vars)?;
                Ok(left_val - right_val)
            }
            Expr::Mul(left, right) => {
                let left_val = left.eval(vars)?;
                let right_val = right.eval(vars)?;
                Ok(left_val * right_val)
            }
            Expr::Div(left, right) => {
                let left_val = left.eval(vars)?;
                let right_val = right.eval(vars)?;
                if right_val == 0.0 {
                    Err("Division by zero".into())
                } else {
                    Ok(left_val / right_val)
                }
            }
        }
    }
}

fn main() {
    
    let expr = Expr::Add(
        Box::new(Expr::Const(2.0)),
        Box::new(Expr::Mul(
            Box::new(Expr::Var("x".to_string())),
            Box::new(Expr::Const(3.0)),
        )),
    );
    
    
    println!("{}", expr.display());  

    let mut vars = HashMap::new();
    vars.insert("x".to_string(), 5.0);

    match expr.eval(&vars) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

