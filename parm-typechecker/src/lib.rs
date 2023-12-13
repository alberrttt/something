use parm_ast::source_file::SourceFile;

#[derive(Debug)]
pub struct Typechecker {}
#[derive(Debug)]
pub struct Scope {}

#[derive(Debug)]
pub struct TypeCheckedSourceFile<'a> {
    pub source_file: SourceFile<'a>,
}

impl<'a> TypeCheckedSourceFile<'a> {
    pub fn new(source_file: SourceFile<'a>) -> Self {
        Self { source_file }
    }
    pub fn typecheck(&mut self) {}
}

fn tc_item() {}
