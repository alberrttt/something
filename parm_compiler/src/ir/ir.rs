use crate::ir::state::RegIdx;

#[derive(Debug, Clone, PartialEq)]
pub enum IR {
    Variable(RegIdx),
    Push {
        value: IRValue,
    },
    Pop {
        into: RegIdx,
    },

    /// pops into nowhere
    PopNoWhere,
    Move {
        from: IRValue,
        into: RegIdx,
    },
    PrintLn {
        from: RegIdx,
    },
    Add {
        lhs: RegIdx,
        rhs: RegIdx,
        into: RegIdx,
    },
    Sub {
        lhs: RegIdx,
        rhs: RegIdx,
        into: RegIdx,
    },
    Mul {
        lhs: RegIdx,
        rhs: RegIdx,
        into: RegIdx,
    },
    Div {
        lhs: RegIdx,
        rhs: RegIdx,
        into: RegIdx,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum IRValue {
    Register(RegIdx),
    Float(f64),
}
