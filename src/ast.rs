pub enum Bop{
    Plus,
    Minus,
    Mult,
    Div
}

pub enum Factor{
    Int(i32),
    Expression(Box<Expression>)
}


pub enum Term{
    Factor(Factor),
    ScalingOperation(Factor, Bop, Factor)
}


pub enum Expression{
    Term(Term),
    AdditiveOperation(Term, Bop, Box<Expression>)
}

use Expression::*;
use Term::*;
use Factor::*;
use Bop::*;

fn evaluate_operator(op: Bop, v1: i32, v2: i32) -> i32{
    match op {
        Plus    => v1 + v2,
        Minus   => v1 - v2,
        Mult    => v1 * v2,
        Div     => v1 / v2
    }
}

fn evaluate_factor(fact: Factor) -> i32{
    match fact{
        Int(n) => n,
        Expression(expr) => evaluate(*expr)
    }
}

fn evaluate_term(term: Term) -> i32{
    match term{
        Factor(fact) => evaluate_factor(fact),
        ScalingOperation(f1, op, f2) => evaluate_operator(op, evaluate_factor(f1), evaluate_factor(f2))
    }
}

pub fn evaluate(expression: Expression) -> i32{
    match expression {
        Term(term) => evaluate_term(term),
        AdditiveOperation(term, op, expr) => evaluate_operator(op, evaluate_term(term), evaluate(*expr))
    }
}