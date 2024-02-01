use super::{registers::Register, value::Value};

#[derive(Debug, Clone, PartialEq)]
pub enum IRCode {
    LoadValue {
        from: Box<Value>,
        into: u8,
    },
    Move {
        from: u8,
        into: u8,
    },
    Print {
        value: u8,
    },
    Add {
        lhs: u8,
        rhs: u8,
        into: u8,
    },
    /// Add the value of `rhs` to `lhs` and store the result in `lhs`
    AddInPlace {
        lhs: u8,
        rhs: u8,
    },
}
