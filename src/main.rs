mod ast;
mod lexer;
mod parser;

fn main() {
    println!("Insert expression: ");
    //must_interpreter::interprete();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to open stdin.");

    let tokens = lexer::parse_input(&input);

    print!("Recognized tokens: ");
    for token in &tokens{
        print!("{token:?}, ")
    }
    println!();

    let expression = match parser::parse(tokens){
        Ok(expr) => expr,
        Err(what) => {
            println!("{what}");
            return;
        }
    };

    let value = ast::evaluate(expression);

    println!("{value:?}");
}
