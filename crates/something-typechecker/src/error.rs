use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum TypeError {
    MismatchedTypes {
        expected: TypeSig,
        found: TypeSig,
    },
    IncorrectTypeName {
        expected: &'static str,
        found: String,
    },
}
impl TypeError {
    pub fn mismatched(expected: TypeSig, found: TypeSig) -> Self {
        Self::MismatchedTypes { expected, found }
    }
}
use colored::*;

use crate::types::sig::TypeSig;
impl std::error::Error for TypeError {}
impl Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TypeError::*;
        match self {
            MismatchedTypes { expected, found } => {
                write!(
                    f,
                    "{} {} {} {}",
                    "Expected".red().bold(),
                    expected.to_string().yellow(),
                    "but found".red().bold(),
                    found.to_string().yellow()
                )
            }
            IncorrectTypeName { expected, found } => {
                write!(
                    f,
                    "{} {} {} {}",
                    "Expected".red().bold(),
                    expected.yellow(),
                    "but found".red().bold(),
                    found.yellow()
                )
            }
        }
    }
}
