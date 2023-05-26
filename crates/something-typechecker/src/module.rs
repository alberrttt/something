use something_frontend::Ident;

use crate::prelude::{FnCtx, Type};
#[derive(Debug, Clone)]
pub enum ModuleItem {
    Fn(FnCtx),
    Static(Ident, Type),
    SubModule(Box<Module>),
}
#[derive(Debug, Clone)]
pub struct Module {
    pub items: Vec<ModuleItem>,
}
