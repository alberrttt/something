use std::fmt::Display;

use crate::prelude::Type;
#[derive(Debug, Clone)]
pub enum TypeError {
    MismatchedTypes { expected: Type, found: Type },
}
impl TypeError {
    pub fn mismatched(expected: Type, found: Type) -> Self {
        Self::MismatchedTypes { expected, found }
    }
}
use colored::*;
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
        }
    }
}
