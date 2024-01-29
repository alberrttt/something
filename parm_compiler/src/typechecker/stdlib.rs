use std::{cell::UnsafeCell, rc::Rc};

use super::{
    symbol::{InnerSymbol, Symbol},
    ty::Type,
    Typechecker,
};

impl<'a> Typechecker<'a> {
    pub fn load_stdlib<'b: 'a>(&'b mut self) {
        let mut tc = UnsafeCell::new(self);
        let ret = Type {
            data: super::ty::TypeData::None,
        }
        .allocate(&mut unsafe { &mut *tc.get() }.ty_arena);
        let top = unsafe { &mut *tc.get() }.scopes.arena.get_mut(0).unwrap();
        let mut top = top.borrow_mut();
        top.vars.insert(
            "println",
            Symbol {
                inner: Rc::new(InnerSymbol {
                    source_file: unsafe { &mut *tc.get() }.source_file,
                    name: "println",
                    ty: Type {
                        data: super::ty::TypeData::Function {
                            params: Vec::new(),
                            ret: Box::new(ret),
                        },
                    }
                    .allocate(&mut unsafe { &mut *tc.get() }.ty_arena),
                }),
            },
        );
    }
}
