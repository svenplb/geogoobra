use std::collections::HashMap;

mod expr;  

use expr::Expr;
use expr::parser::{tokenize, parse_expression};  



fn main() {
 
    let input = "(2 + x) * 3";
    let tokens = tokenize(input);
    println!("{:?}", tokens); 

    let (expr, _) = parse_expression(&tokens, 0);
    println!("{}", expr.display());

    let mut vars = HashMap::new();
    vars.insert("x".to_string(), 5.0);

    match expr.eval(&vars) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
