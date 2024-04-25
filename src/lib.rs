mod ast;
mod lexer;
mod parser;

pub use crate::ast::Value;

pub fn interprete(input: &String) -> Result<Value, String> {
    ast::evaluate(parser::parse(lexer::parse_input(&input))?)
}
