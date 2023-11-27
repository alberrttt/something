use crate::prelude::*;
use parmesan_ast::prelude::*;
type Function<'a> = crate::prelude::Function<'a>;
impl<'a> Lower for parmesan_ast::prelude::Function<'a> {
    type Output = Function<'a>;

    fn parse(&self, ctx: &mut crate::lowering::LoweringContext) -> Self::Output {
        let function = Function::new(self.name.lexeme);
        for statement in self.body.iter() {
            match statement {
                Item::Variable(variable) => variable.lower(),
                Item::Function(_) => todo!(),
                Item::Statement(_) => todo!(),
            }
        }
        function
    }
}
impl<'a> Lower for &'a Variable<'a> {
    type Output = Operand<'a>;

    fn parse(&self, ctx: &mut crate::lowering::LoweringContext) -> Self::Output {
        let scope = ctx.scopes.last_mut().unwrap();
        let idx = scope.variables.len();
        scope.variables.push(self.ident);
        match self.initializer {
            Some(initializer) => return Operand::Local(None),
            None => return Operand::Local(None),
        }
    }
}
impl<'a> Lower for Statement<'a> {
    type Output = Operand<'a>;

    fn parse(&self, ctx: &mut crate::lowering::LoweringContext) -> Self::Output {
        todo!()
    }
}
