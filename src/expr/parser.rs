
use crate::expr::Expr;

#[derive(Debug, PartialEq)]
pub enum Token{
    Number(f64),
    Variable(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

// aka lexer
// convert string into vec of tokens
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    // Loop through all chars and assign them to some Token / ignore spaces
    while let Some(&ch) = chars.peek() {
        // digit
        match ch {
            '0'..='9' | '.' => {
                let mut number = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_numeric() || ch == '.' {
                        number.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number.parse().unwrap()));
            }
            // operators
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Star);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Slash);
                chars.next();
            }
            // parenth
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            // vars
            'a'..='z' | 'A'..='Z' => {
                let mut var = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphabetic() {
                        var.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Variable(var));
            }
            ' ' => { chars.next(); } // ignore / skip spaces
            _ => panic!("Unexpected character: {}", ch),
        }
    }
    tokens

}

// I hate this... NOTES:
/*
recursive descent parser // top down parser.
Kind of like a chess search tree, but not really. Still cool

Split up into these 3 to respsect operator precedence:
parsing expressions fn:  addition and subtraction, calls parse term
parsing term fn: handling multipliaction and division, calls parse factor
parse factor fn: numbers, variables, (), calls parse primary

parse primary: basic stuff, num, var parneth

*/

// subtraction / addition
pub fn parse_expression(tokens: &[Token], pos: usize) -> (Expr, usize) {
    parse_term(tokens, pos)
}


// multiplication / division
fn parse_term(tokens: &[Token], pos: usize) -> (Expr, usize) {
    let (mut left_expr, mut pos) = parse_factor(tokens, pos);

       while pos < tokens.len() {
        match tokens[pos] {
            Token::Plus => {
                let (right_expr, new_pos) = parse_factor(tokens, pos + 1);
                left_expr = Expr::Add(Box::new(left_expr), Box::new(right_expr));
                pos = new_pos;
            }
            Token::Minus => {
                let (right_expr, new_pos) = parse_factor(tokens, pos + 1);
                left_expr = Expr::Sub(Box::new(left_expr), Box::new(right_expr));
                pos = new_pos;
            }
            _ => break,
        }
    }
    (left_expr, pos)


}

fn parse_factor(tokens: &[Token], pos: usize) -> (Expr, usize) {
    let (mut left_expr, mut pos) = parse_primary(tokens, pos);

    while pos < tokens.len() {
        match tokens[pos] {
            Token::Star => {
                let (right_expr, new_pos) = parse_primary(tokens, pos + 1);
                left_expr = Expr::Mul(Box::new(left_expr), Box::new(right_expr));
                pos = new_pos;
            }
            Token::Slash => {
                let (right_expr, new_pos) = parse_primary(tokens, pos + 1);
                left_expr = Expr::Div(Box::new(left_expr), Box::new(right_expr));
                pos = new_pos;
            }
            _ => break,
        }
    }
    (left_expr, pos)
}

fn parse_primary(tokens: &[Token], pos: usize) -> (Expr, usize) {
    match &tokens[pos] {
        Token::Number(n) => (Expr::Const(*n), pos + 1),
        Token::Variable(name) => (Expr::Var(name.clone()), pos + 1),
        Token::LParen => {
            let (expr, new_pos) = parse_expression(tokens, pos + 1); // No slicing, use the full tokens
            if new_pos < tokens.len() && tokens[new_pos] == Token::RParen {
                (expr, new_pos + 1) // Move past the closing parenthesis
            } else {
                panic!("Expected closing parenthesis");
            }
        }
        _ => panic!("Unexpected token: {:?}", tokens[pos]),
    }
}



