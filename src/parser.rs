use crate::ast::Term::*;
use crate::ast::Factor::*;
use crate::ast::Bop::*;
use crate::ast::Expression::*;

use crate::lexer::Token::*;
use crate::lexer::Token;

use crate::ast::Expression;
use crate::ast::Term;
use crate::ast::Factor;

use std::iter::Peekable;

fn parse_factor(it: &mut Peekable<impl Iterator<Item=Token>>) -> Result<Factor, String>{
    match it.next(){
        Some(INT(n)) => return Ok(Int(n)),
        Some(LPAREN) => {
            let expr = Expression(Box::new(parse_expression(it)?));
            match it.next(){
                Some(RPAREN) => Ok(expr),
                Some(other) => Err(format!("Unexpected token: {:?}", other)),
                None => Err(format!("Expected token: RPAREN")),
            }
        }
        Some(other) => Err(format!("Unexpected token: {:?}", other)),
        None => Err(format!("Expected token")),
    }
}

fn parse_term(it: &mut Peekable<impl Iterator<Item=Token>>) -> Result<Term, String>{
    let factor = parse_factor(it)?;

    match it.peek(){
        | Some(MULT) 
        | Some(DIV) => {
            let op = match it.next().unwrap(){
                MULT    => Mult,
                DIV     => Div,
                _       => unreachable!()
            };
            return Ok(ScalingOperation(factor, op, parse_factor(it)?));
        }
        _ => Ok(Factor(factor))
    }
}

fn parse_expression(it: &mut Peekable<impl Iterator<Item=Token>>) -> Result<Expression, String>{
    let term = parse_term(it)?;

    match it.peek(){
        | Some(PLUS) 
        | Some(MINUS) => {
            let op = match it.next().unwrap(){
                PLUS    => Plus,
                MINUS   => Minus,
                _       => unreachable!()
            };
            return Ok(AdditiveOperation(term, op, Box::new(parse_expression(it)?)));
        },
        | Some(RPAREN)
        | None => Ok(Term(term)),
        Some(other) => Err(format!("Unexpected token: {:?}", other))
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Expression, String>{
    let mut peekable_tokens = tokens.into_iter().peekable();
    let result = parse_expression(&mut peekable_tokens);

    if peekable_tokens.len() > 0{
        Err(format!("Unexpected token: {:?}", peekable_tokens.peek().unwrap()))
    }
    else{
        result
    }
}