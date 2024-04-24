use std::ops::*;

#[derive(Debug)]
pub enum Value{
    Int(i128),
    Float(f64)
}

use Value::*;

impl Add for Value{
    type Output = Value;

    fn add(self, other: Self) -> Self::Output{
        match self{
            Int(n) => {
                match other{
                    Int(m) => Int(n + m),
                    Float(m) => Float(n as f64 + m)
                }
            }
            Float(n) => {
                match other{
                    Int(m) => Float(n + m as f64),
                    Float(m) => Float(n + m)
                }
            }
        }
    }
}

impl Sub for Value{
    type Output = Value;

    fn sub(self, other: Self) -> Self::Output{
        match self{
            Int(n) => {
                match other{
                    Int(m) => Int(n - m),
                    Float(m) => Float(n as f64 - m)
                }
            }
            Float(n) => {
                match other{
                    Int(m) => Float(n - m as f64),
                    Float(m) => Float(n - m)
                }
            }
        }
    }
}

impl Mul for Value{
    type Output = Value;

    fn mul(self, other: Self) -> Self::Output{
        match self{
            Int(n) => {
                match other{
                    Int(m) => Int(n * m),
                    Float(m) => Float(n as f64 * m)
                }
            }
            Float(n) => {
                match other{
                    Int(m) => Float(n * m as f64),
                    Float(m) => Float(n * m)
                }
            }
        }
    }
}

impl Div for Value{
    type Output = Value;

    fn div(self, other: Self) -> Self::Output{
        match self{
            Int(n) => {
                match other{
                    Int(m) => if n % m == 0 { Int(n / m) } else { Float(n as f64 / m as f64) }
                    Float(m) => Float(n as f64 / m)
                }
            }
            Float(n) => {
                match other{
                    Int(m) => Float(n * m as f64),
                    Float(m) => Float(n * m)
                }
            }
        }
    }
}

impl Value {
    pub fn pow(self, other: Self) -> Value{
        match self{
            Int(n) => {
                match other{
                    Int(m) => Int(n.pow(m as u32)),
                    Float(m) => Float((n as f64).powf(m)),
                }
            }
            Float(n) => {
                match other{
                    Int(m) => Float(n.powf(m as f64 )),
                    Float(m) => Float(n.powf(m)),
                }
            }
        }
    }
}