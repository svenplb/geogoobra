
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

}
