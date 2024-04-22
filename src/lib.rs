mod lexer;
mod parser;
mod ast;

pub fn interprete(input: &String) -> Result<i32, String>{
    Ok(ast::evaluate(parser::parse(lexer::parse_input(&input))?))
}