mod value;
pub use value::Value;

#[derive(Debug)]
pub enum Bop {
    Plus,
    Minus,
    Mult,
    Div,
    Pow,
}

#[derive(Debug)]
pub enum Factor {
    Number(Value),
    Expression(Box<Expression>),
}

#[derive(Debug)]
pub enum Power {
    Factor(Factor),
    PowerOperation(Box<Power>, Bop, Factor),
}

#[derive(Debug)]
pub enum Term {
    Power(Power),
    ScalingOperation(Box<Term>, Bop, Power),
}

#[derive(Debug)]
pub enum Expression {
    Term(Term),
    AdditiveOperation(Box<Expression>, Bop, Term),
}

use Bop::*;

fn evaluate_operator(op: Bop, v1: Value, v2: Value) -> Result<Value, String> {
    match op {
        Plus => v1 + v2,
        Minus => v1 - v2,
        Mult => v1 * v2,
        Div => v1 / v2,
        Pow => v1.pow(v2),
    }
}

fn evaluate_factor(fact: Factor) -> Result<Value, String> {
    match fact {
        Factor::Number(n) => Ok(n),
        Factor::Expression(expr) => evaluate(*expr),
    }
}

fn evaluate_power(power: Power) -> Result<Value, String> {
    match power {
        Power::Factor(fact) => evaluate_factor(fact),
        Power::PowerOperation(pow, op, fact) => {
            evaluate_operator(op, evaluate_power(*pow)?, evaluate_factor(fact)?)
        }
    }
}

fn evaluate_term(term: Term) -> Result<Value, String> {
    match term {
        Term::Power(pow) => evaluate_power(pow),
        Term::ScalingOperation(term, op, pow) => {
            evaluate_operator(op, evaluate_term(*term)?, evaluate_power(pow)?)
        }
    }
}

pub fn evaluate(expression: Expression) -> Result<Value, String> {
    match expression {
        Expression::Term(term) => evaluate_term(term),
        Expression::AdditiveOperation(expr, op, term) => {
            evaluate_operator(op, evaluate(*expr)?, evaluate_term(term)?)
        }
    }
}
