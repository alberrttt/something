use std::{cell::UnsafeCell, collections::HashMap};

use crate::{ast::prelude::*, typechecker::Typechecker};

use super::{
    code::IRCode,
    function::IRFunction,
    registers::{Register, Registers},
    value::Value,
};

#[derive(Debug, Clone)]
pub struct Lowering<'a> {
    pub registers: Registers,
    pub vars: HashMap<&'a str, u8>,
    pub tc: &'a Typechecker<'a>,
}

impl<'a> Lowering<'a> {
    pub fn lower(&mut self) -> IRFunction {
        let mut ir_function = IRFunction { code: vec![] };
        let lowering = UnsafeCell::new(self);
        for item in &unsafe { &*lowering.get() }.tc.source_file.ast {
            let Item::Function(fnc) = item else {
                continue;
            };
            if fnc.name.lexeme != "main" {
                continue;
            }
            for statement in &fnc.body.statements.inner {
                ir_function
                    .code
                    .extend(statement.lower(unsafe { *lowering.get() }));
            }
        }
        ir_function
    }
}
