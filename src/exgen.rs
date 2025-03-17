use rand::{Rng, SeedableRng, rngs::StdRng};
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq)]
pub struct BinaryOpArgs {
    a: u16,
    b: u16,
}

pub trait AddFormat {
    fn fmt_add(&self) -> String;
}

pub trait SubFormat {
    fn fmt_sub(&self) -> String;
}

pub trait MulFormat {
    fn fmt_mul(&self) -> String;
}

pub trait DivFormat {
    fn fmt_div(&self) -> String;
}

impl AddFormat for BinaryOpArgs {
    fn fmt_add(&self) -> String {
        format!("{} + {}", self.a, self.b)
    }
}

impl SubFormat for BinaryOpArgs {
    fn fmt_sub(&self) -> String {
        if self.a > self.b {
            format!("{} - {}", self.a, self.b)
        } else {
            format!("{} - {}", self.b, self.a)
        }
    }
}

impl MulFormat for BinaryOpArgs {
    fn fmt_mul(&self) -> String {
        format!("{} * {}", self.a, self.b)
    }
}

impl DivFormat for BinaryOpArgs {
    fn fmt_div(&self) -> String {
        if self.a != self.b {
            format!(
                "{} / {}\n{} / {}",
                self.a * self.b,
                self.a,
                self.a * self.b,
                self.b
            )
        } else {
            format!("{} / {}", self.a * self.b, self.a)
        }
    }
}

pub fn generate_excercises(arg_limit: u16, excercise_number: usize) -> HashSet<BinaryOpArgs> {
    let mut rng = StdRng::from_os_rng();
    let mut result = HashSet::new();
    while result.len() < excercise_number {
        let a = rng.random_range(1..=arg_limit);
        let b = rng.random_range(1..=arg_limit);
        result.insert(BinaryOpArgs { a, b });
    }
    result
}
