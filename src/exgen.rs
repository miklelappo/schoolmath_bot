use rand::{Rng, SeedableRng, distr::StandardUniform, prelude::Distribution, rngs::StdRng};
use std::{
    cmp::{max, min},
    collections::HashSet,
    fmt,
    ops::RangeInclusive,
};

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum OpSign {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum DisplayMode {
    Exercise,     // a <op> b =
    MissingLeft,  // ? <op> b = result
    MissingRight, // a <op> ? = result
}

impl Distribution<DisplayMode> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DisplayMode {
        match rng.random_range(0..=2) {
            // rand 0.8
            0 => DisplayMode::Exercise,
            1 => DisplayMode::MissingLeft,
            _ => DisplayMode::MissingRight,
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct BinaryOp {
    a: u32,
    b: u32,
    sign: OpSign,
    result: u32,
    mode: DisplayMode,
}

impl fmt::Display for OpSign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpSign::Add => write!(f, "+"),
            OpSign::Sub => write!(f, "-"),
            OpSign::Mul => write!(f, "x"),
            OpSign::Div => write!(f, ":"),
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.mode {
            DisplayMode::Exercise => write!(f, "{} {} {} = ?", self.a, self.sign, self.b),
            DisplayMode::MissingLeft => write!(f, "? {} {} = {}", self.sign, self.b, self.result),
            DisplayMode::MissingRight => write!(f, "{} {} ? = {}", self.a, self.sign, self.result),
        }
    }
}

impl BinaryOp {
    pub fn a(&self) -> u32 { self.a }
    pub fn b(&self) -> u32 { self.b }
    pub fn sign(&self) -> OpSign { self.sign }
}

pub fn generate_excercises(
    sign: OpSign,
    arg_limit: RangeInclusive<u32>,
    excercise_number: usize,
) -> HashSet<BinaryOp> {
    let mut rng = StdRng::from_os_rng();
    let mut result = HashSet::new();
    while result.len() < excercise_number {
        let a = rng.random_range(arg_limit.clone());
        let b = rng.random_range(arg_limit.clone());
        match sign {
            OpSign::Add => {
                result.insert(BinaryOp {
                    a,
                    b,
                    sign,
                    result: a + b,
                    mode: rand::random(),
                });
            }
            OpSign::Sub => {
                result.insert(BinaryOp {
                    a: max(a, b),
                    b: min(a, b),
                    sign,
                    result: a.max(b) - a.min(b),
                    mode: rand::random(),
                });
            }
            OpSign::Mul => {
                result.insert(BinaryOp {
                    a,
                    b,
                    sign,
                    result: a * b,
                    mode: DisplayMode::Exercise,
                });
            }
            OpSign::Div => {
                if a == b {
                    result.insert(BinaryOp {
                        a: a * b,
                        b,
                        sign,
                        result: a,
                        mode: DisplayMode::Exercise,
                    });
                } else {
                    result.insert(BinaryOp {
                        a: a * b,
                        b,
                        sign,
                        result: a,
                        mode: DisplayMode::Exercise,
                    });
                }
            }
        }
    }
    result
}
