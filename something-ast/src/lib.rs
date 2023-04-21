use declaration::Declaration;

pub struct Ast {
    pub nodes: Vec<Node>,
}

pub enum Node {
    Declaration(Declaration),
}

pub mod declaration;
