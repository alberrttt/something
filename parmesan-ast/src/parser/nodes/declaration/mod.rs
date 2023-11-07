mod variable;

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Variable(variable::Variable),
}
