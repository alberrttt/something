use std::{collections::HashMap, rc::Rc};

use crate::{prelude::Type, symbol::Symbol};

pub struct Context {
    pub vars: HashMap<Rc<Symbol>, Type>,
}
