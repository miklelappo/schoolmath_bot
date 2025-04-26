use rand::{Rng, SeedableRng, rngs::StdRng};
use std::{
    cmp::{max, min},
    collections::HashSet,
    fmt,
};

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum OpSign {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Hash, Eq, PartialEq)]
pub struct BinaryOp {
    a: u16,
    b: u16,
    sign: OpSign,
    result: u16,
}

impl fmt::Display for OpSign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpSign::Add => write!(f, "+"),
            OpSign::Sub => write!(f, "-"),
            OpSign::Mul => write!(f, "*"),
            OpSign::Div => write!(f, ":"),
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} =", self.a, self.sign, self.b)
    }
}

pub fn generate_excercises(
    sign: OpSign,
    arg_limit: u16,
    excercise_number: usize,
) -> HashSet<BinaryOp> {
    let mut rng = StdRng::from_os_rng();
    let mut result = HashSet::new();
    while result.len() < excercise_number {
        let a = rng.random_range(1..=arg_limit);
        let b = rng.random_range(1..=arg_limit);
        match sign {
            OpSign::Add => {
                result.insert(BinaryOp {
                    a,
                    b,
                    sign,
                    result: a + b,
                });
            }
            OpSign::Sub => {
                result.insert(BinaryOp {
                    a: max(a, b),
                    b: min(a, b),
                    sign,
                    result: (a as i16 - b as i16).unsigned_abs(),
                });
            }
            OpSign::Mul => {
                result.insert(BinaryOp {
                    a,
                    b,
                    sign,
                    result: a * b,
                });
            }
            OpSign::Div => {
                if a == b {
                    result.insert(BinaryOp {
                        a: a * b,
                        b,
                        sign,
                        result: a,
                    });
                } else {
                    result.insert(BinaryOp {
                        a: a * b,
                        b,
                        sign,
                        result: a,
                    });
                    result.insert(BinaryOp {
                        a: a * b,
                        b: a,
                        sign,
                        result: b,
                    });
                }
            }
        }
    }
    result
}
