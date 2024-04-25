use crate::ast::Bop::*;
use crate::ast::Expression::*;
use crate::ast::Factor::*;
use crate::ast::Power::*;
use crate::ast::Term::*;

use crate::lexer::Token;
use crate::lexer::Token::*;

use crate::ast::Expression;
use crate::ast::Factor;
use crate::ast::Power;
use crate::ast::Term;
use crate::ast::Value;

use std::iter::Peekable;

fn parse_factor(it: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Factor, String> {
    match it.next() {
        Some(INT(n)) => Ok(Number(Value::Int(n))),
        Some(FLOAT(n)) => Ok(Number(Value::Float(n))),
        Some(MINUS) => match it.next() {
            Some(INT(n)) => Ok(Number(Value::Int(-n))),
            Some(FLOAT(n)) => Ok(Number(Value::Float(-n))),
            Some(other) => Err(format!("Unexpected token: {:?}", other)),
            None => Err(format!("Expected value")),
        },
        Some(LPAREN) => {
            let expr = Expression(Box::new(parse_expression(it)?));
            match it.next() {
                Some(RPAREN) => Ok(expr),
                Some(other) => Err(format!("Unexpected token: {:?}", other)),
                None => Err(format!("Expected token: RPAREN")),
            }
        }
        Some(other) => Err(format!("Unexpected token: {:?}", other)),
        None => Err(format!("Expected token")),
    }
}

fn parse_power(it: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Power, String> {
    let mut power = Factor(parse_factor(it)?);

    while let Some(POWER) = it.peek() {
        it.next();
        let factor = parse_factor(it)?;
        power = PowerOperation(Box::new(power), Pow, factor);
    }

    Ok(power)
}

fn parse_term(it: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Term, String> {
    let mut term = Power(parse_power(it)?);

    while let Some(MULT) | Some(DIV) = it.peek() {
        let op = match it.next().unwrap() {
            MULT => Mult,
            DIV => Div,
            _ => unreachable!(),
        };
        let power = parse_power(it)?;
        term = ScalingOperation(Box::new(term), op, power);
    }

    Ok(term)
}

fn parse_expression(it: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expression, String> {
    let mut expr = Term(parse_term(it)?);

    while let Some(PLUS) | Some(MINUS) = it.peek() {
        let op = match it.next().unwrap() {
            PLUS => Plus,
            MINUS => Minus,
            _ => unreachable!(),
        };
        let term = parse_term(it)?;
        expr = AdditiveOperation(Box::new(expr), op, term);
    }

    Ok(expr)
}

pub fn parse(tokens: Vec<Token>) -> Result<Expression, String> {
    let mut peekable_tokens = tokens.into_iter().peekable();
    let result = parse_expression(&mut peekable_tokens);

    if peekable_tokens.len() > 0 {
        Err(format!(
            "Unexpected token: {:?}",
            peekable_tokens.peek().unwrap()
        ))
    } else {
        result
    }
}
