// Idk what to called the AST displayer, so
use std::fmt::{Display, Write};

use crate::ast::{
    parser::nodes::Empty,
    prelude::{BinaryExpression, BinaryOperator, ExpectedNode, Expression, Number, Plus, Token},
};
#[derive(Debug, Clone, PartialEq)]
pub struct Tree {
    pub label: String,
    pub children: Vec<Tree>,
    pub lexeme: String,
}
const EMPTY: &str = "   ";
const EDGE: &str = " └──";
const PIPE: &str = " │ ";
const BRANCH: &str = " ├──";
impl Tree {
    pub fn generate(&self) -> String {
        let mut result = String::new();
        writeln!(result, "{} {}", self.label, self.lexeme).unwrap();
        for (index, child) in self.children.iter().enumerate() {
            let is_last = index == self.children.len() - 1;
            result.push_str(&child.generate_helper(is_last, ""));
        }
        result
    }
    pub fn generate_helper(&self, is_last: bool, prefix: &str) -> String {
        let mut result = String::new();
        let marker = if is_last { EDGE } else { BRANCH };

        writeln!(result, "{prefix}{marker}{} {}", self.label, self.lexeme).unwrap();
        let new_prefix = format!("{}{}", prefix, if is_last { EMPTY } else { PIPE });
        for (index, child) in self.children.iter().enumerate() {
            let is_last = index == self.children.len() - 1;
            result.push_str(&child.generate_helper(is_last, &new_prefix));
        }
        result
    }
    pub fn new(label: impl Display) -> Self {
        Self {
            label: label.to_string(),
            children: vec![],
            lexeme: "".to_string(),
        }
    }
    pub fn child(mut self, child: Tree) -> Self {
        self.children.push(child);
        self
    }
    pub fn label(mut self, label: impl Display) -> Self {
        self.label = format!("{label}: ") + self.label.as_str();
        self
    }
    pub fn lexeme(mut self, lexeme: impl Display) -> Self {
        self.lexeme = lexeme.to_string();
        self
    }
}
impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.generate())?;
        Ok(())
    }
}
pub trait TreeDisplay {
    fn tree(&self) -> Tree;
}

impl<T: TreeDisplay> TreeDisplay for Box<T> {
    fn tree(&self) -> Tree {
        (**self).tree()
    }
}
impl<T: TreeDisplay> TreeDisplay for Option<T> {
    fn tree(&self) -> Tree {
        match self {
            Some(t) => t.tree(),
            None => Tree::new("None"),
        }
    }
}
impl<T: TreeDisplay> TreeDisplay for Vec<T> {
    fn tree(&self) -> Tree {
        let mut tree = Tree::new("Vec");
        for item in self {
            tree = tree.child(item.tree());
        }
        tree
    }
}
#[test]
fn test_tree() {
    let token = &Token::default();
    let bin = BinaryExpression {
        left: Box::new(Expression::Number(Number { token, value: 1.0 })),
        operator: BinaryOperator::Plus(Plus::default()),
        right: Box::new(Expression::BinaryExpression(BinaryExpression {
            left: Box::new(Expression::Number(Number { token, value: 2.0 })),
            operator: BinaryOperator::Plus(Plus::default()),
            right: Box::new(Expression::Number(Number { token, value: 3.0 })),
        })),
    };

    let tree = bin.tree();
    println!("{}", tree);
}
// AST:
// BinaryExpression
// ├─ left: Expression
// ├─ right: BinaryExpression
// │   ├─ left: Expression
// │   ├─ right: Expression
// │   └─ operator: Operator +
// └─ operator: +
