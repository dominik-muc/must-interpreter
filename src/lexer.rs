#[derive(Debug, PartialEq, Clone)]
pub enum Token{
    INT(i128),
    FLOAT(f64),
    STRING(String),
    PLUS,
    MINUS,
    MULT,
    DIV,
    LPAREN,
    RPAREN,
    POWER
}

use Token::*;

fn parse_token(c: char) -> Option<Token>{
    match c{
        ' ' | '\n' | '\t' => None,
        '+' => Some(PLUS),
        '-' => Some(MINUS),
        '*' => Some(MULT),
        '/' => Some(DIV),
        '(' => Some(LPAREN),
        ')' => Some(RPAREN),
        '^' => Some(POWER),
        _ => unreachable!()
    }
}

fn parse_buffer(buffer: &String, tokens: &mut Vec<Token>) {
    match buffer.parse(){
        Ok(n) => tokens.push(INT(n)),
        Err(_) => {
            match buffer.parse(){
                Ok(m) => tokens.push(FLOAT(m)),
                Err(_) => match buffer.as_str(){
                    "" => (),
                    _ => tokens.push(STRING(String::from(buffer)))
                }
            }   
        }
    }
}

pub fn parse_input(input: &String) -> Vec<Token> {
    let mut tokens:Vec<Token>  = Vec::new();
    let mut buffer = String::new();

    for c in input.chars(){
        match c {
            'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '.' => buffer.push(c),
            '+' | '-' | '*' | '/' | '(' | ')' | '^' | ' ' | '\n' | '\t' => {
                parse_buffer(&buffer, &mut tokens);
                buffer = String::new();
                match parse_token(c){
                    Some(token) => tokens.push(token),
                    None => ()
                }
            }
            other => {
                eprintln!("Skipping unknown token: {}", other);
                continue
            }
        };
    }
    // Get last token aswell
    parse_buffer(&buffer, &mut tokens);

    return tokens;
}