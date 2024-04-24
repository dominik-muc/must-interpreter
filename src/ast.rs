mod value;
pub use value::Value;
pub enum Bop{
    Plus,
    Minus,
    Mult,
    Div,
    Pow
}

pub enum Factor{
    Number(Value),
    Expression(Box<Expression>)
}

pub enum Power{
    Factor(Factor),
    PowerOperation(Factor, Bop, Box<Power>),
}

pub enum Term{
    Power(Power),
    ScalingOperation(Power, Bop, Box<Term>)
}


pub enum Expression{
    Term(Term),
    AdditiveOperation(Term, Bop, Box<Expression>)
}


use Expression::*;
use Term::*;
use Power::*;
use Factor::*;
use Bop::*;

fn evaluate_operator(op: Bop, v1: Value, v2: Value) -> Value{
    match op {
        Plus    => v1 + v2,
        Minus   => v1 - v2,
        Mult    => v1 * v2,
        Div     => v1 / v2,
        Pow     => v1.pow(v2)
    }
}

fn evaluate_factor(fact: Factor) -> Value{
    match fact {
        Number(n) => n,
        Expression(expr) => evaluate(*expr)
    }
}

fn evaluate_power(power: Power) -> Value{
    match power {
        Factor(fact) => evaluate_factor(fact),
        PowerOperation(fact, op, pow) => evaluate_operator(op, evaluate_factor(fact), evaluate_power(*pow))
    }
}

fn evaluate_term(term: Term) -> Value{
    match term {
        Power(pow) => evaluate_power(pow),
        ScalingOperation(pow, op, term) => evaluate_operator(op, evaluate_power(pow), evaluate_term(*term))
    }
}

pub fn evaluate(expression: Expression) -> Value{
    match expression {
        Term(term) => evaluate_term(term),
        AdditiveOperation(term, op, expr) => evaluate_operator(op, evaluate_term(term), evaluate(*expr))
    }
}