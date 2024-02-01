use super::code::IRCode;

#[derive(Debug, Clone)]
pub struct IRFunction {
    pub code: Vec<IRCode>,
}
