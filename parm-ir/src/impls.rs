use crate::prelude::*;
use parm_ast::prelude::*;
type Function<'a> = crate::prelude::Function<'a>;
impl<'a> Lower<'a> for parm_ast::prelude::Function<'a> {
    type Output = (Function<'a>, Scope<'a>);

    fn lower(&self, ctx: &mut crate::lowering::LoweringContext<'a>) -> Self::Output {
        let mut function = Function::new(self.name.lexeme);
        ctx.scopes.push(Scope::default());
        for statement in self.body.iter() {
            match statement {
                Item::Variable(variable) => {
                    function.add_operand(variable.lower(ctx));
                }

                _ => todo!(),
            }
        }

        (function, ctx.scopes.pop().unwrap())
    }
}
impl<'a> Lower<'a> for Variable<'a> {
    type Output = Operand<'a>;

    fn lower(&self, ctx: &mut crate::lowering::LoweringContext<'a>) -> Self::Output {
        let scope = ctx.scopes.last_mut().unwrap();
        let idx = scope.variables.len();
        scope.variables.push(self.ident.clone());
        match &self.initializer {
            Some(initializer) => return Operand::Let(None),
            None => return Operand::Let(None),
        }
    }
}

#[test]
fn test() {
    let mut ctx = LoweringContext::default();
    let mut parser = Parser::new("fn main() { let a = 1; } -> a");

    let ast = parm_ast::prelude::Function::parse(&mut parser.stream).unwrap();
    let (function, scope) = ast.lower(&mut ctx);

    dbg!(function);
}
