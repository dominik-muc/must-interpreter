mod lexer;
mod parser;
mod ast;

pub use crate::ast::Value;

pub fn interprete(input: &String) -> Result<Value, String>{
    Ok(ast::evaluate(parser::parse(lexer::parse_input(&input))?))
}