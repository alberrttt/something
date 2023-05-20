use std::fmt::Display;

use colored::Colorize;

use crate::prelude::Type;
#[derive(Debug, Clone)]
pub enum TypeError {
    MismatchedTypes { expected: Type, got: Type },
}

impl Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::MismatchedTypes { expected, got } => {
                write!(f, "{}", "Error ".bold().red())?;
                let args = format!(
                    "{} {expected} {} {got}",
                    "Mismatched types! Got".yellow(),
                    "and".yellow()
                );
                write!(f, "{}", args)
            }
        }
    }
}
impl std::error::Error for TypeError {}
