use std::{fmt::Display, ops::*};

#[derive(Debug)]
pub enum Value {
    Int(i128),
    Float(f64),
}

use Value::*;

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Int(n) => write!(f, "{n}"),
            Float(n) => write!(f, "{n}"),
        }
    }
}

impl Add for Value {
    type Output = Result<Value, String>;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Int(n), Int(m)) => match n.checked_add(m) {
                Some(v) => Ok(Int(v)),
                None => Err(format!("Invalid operation: {} + {}", n, m)),
            },
            (Int(n), Float(m)) => Ok(Float(n as f64 + m)),
            (Float(n), Int(m)) => Ok(Float(n + m as f64)),
            (Float(n), Float(m)) => Ok(Float(n + m)),
        }
    }
}

impl Sub for Value {
    type Output = Result<Value, String>;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Int(n), Int(m)) => match n.checked_sub(m) {
                Some(v) => Ok(Int(v)),
                None => Err(format!("Invalid operation: {} - {}", n, m)),
            },
            (Int(n), Float(m)) => Ok(Float(n as f64 - m)),
            (Float(n), Int(m)) => Ok(Float(n - m as f64)),
            (Float(n), Float(m)) => Ok(Float(n - m)),
        }
    }
}

impl Mul for Value {
    type Output = Result<Value, String>;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Int(n), Int(m)) => match n.checked_mul(m) {
                Some(v) => Ok(Int(v)),
                None => Err(format!("Invalid operation: {} * {}", n, m)),
            },
            (Int(n), Float(m)) => Ok(Float(n as f64 * m)),
            (Float(n), Int(m)) => Ok(Float(n * m as f64)),
            (Float(n), Float(m)) => Ok(Float(n * m)),
        }
    }
}

impl Div for Value {
    type Output = Result<Value, String>;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Int(n), Int(m)) => {
                if m != 0 && n % m != 0 {
                    Ok(Float(n as f64 / m as f64))
                } else {
                    match n.checked_div(m) {
                        Some(v) => Ok(Int(v)),
                        None => Err(format!("Invalid operation: {} / {}", n, m)),
                    }
                }
            }
            (Int(n), Float(m)) => Ok(Float(n as f64 / m)),
            (Float(n), Int(m)) => Ok(Float(n / m as f64)),
            (Float(n), Float(m)) => Ok(Float(n / m)),
        }
    }
}

impl Value {
    pub fn pow(self, other: Self) -> Result<Value, String> {
        match (self, other) {
            (Int(n), Int(m)) => match n.checked_pow(m as u32) {
                Some(v) => Ok(Int(v)),
                None => Err(format!("Invalid operation: {} ^ {}", n, m)),
            },
            (Int(n), Float(m)) => Ok(Float((n as f64).powf(m))),
            (Float(n), Int(m)) => Ok(Float(n.powf(m as f64))),
            (Float(n), Float(m)) => Ok(Float(n.powf(m))),
        }
    }
}
