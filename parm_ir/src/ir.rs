use crate::state::RegIdx;

#[derive(Debug, Clone, PartialEq)]
pub enum IR {
    Variable(RegIdx),
    Push {
        value: IRValue,
    },
    Pop {
        into: RegIdx,
    },
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
    Constant(f64),
}
