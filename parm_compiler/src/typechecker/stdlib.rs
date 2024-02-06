use std::{
    cell::{RefCell, UnsafeCell},
    rc::Rc,
};

use super::{
    symbol::{InnerSymbol, Symbol},
    ty::{Type, TypeData},
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
        let symbol = Symbol::new(
            "println",
            Type {
                data: super::ty::TypeData::Function {
                    params: vec![Type {
                        data: TypeData::Any,
                    }
                    .allocate(&mut unsafe { &mut *tc.get() }.ty_arena)],
                    ret: Box::new(ret),
                },
            }
            .allocate(&mut unsafe { &mut *tc.get() }.ty_arena),
            unsafe { &mut *tc.get() }.source_file,
        );
        top.vars.insert("println", symbol);
    }
}
