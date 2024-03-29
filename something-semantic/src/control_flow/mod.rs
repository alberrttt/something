use something_ast::tokenizer::Parse;

// TODO: learn how control flow works in a compiler
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Flow {
    Continue,
    Break,
    Return,
    Branch(Branch),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Branch {
    stmts: Vec<Flow>,
}
trait ControlFlow {
    fn control_flow(&self) -> Flow;
}
