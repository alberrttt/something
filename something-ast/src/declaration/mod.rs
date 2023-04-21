use self::var::VariableDeclaration;

pub enum Declaration {
    Var(VariableDeclaration),
}
pub mod var;
