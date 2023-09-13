pub mod fnc;
pub mod var;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    Function(fnc::Function),
    Variable(var::Variable),
}
