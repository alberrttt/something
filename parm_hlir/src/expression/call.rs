use parm_common::Spanned;

use crate::{error::TypeError, traits::Check, ty::Type};

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Call<'a, 'b> {
    pub callee: Box<Expression<'a, 'b>>,
    pub arguments: Vec<Expression<'a, 'b>>,
}

impl<'a, 'b> Check<'a, 'b, Call<'a, 'b>> for crate::prelude::ast::Call<'a> {
    fn check(
        &'b self,
        tc: &mut crate::typechecker::Typechecker<'a, 'b>,
    ) -> crate::traits::TypeCheckResult<'a, 'b, Call<'a, 'b>> {
        let callee: Expression = self.callee.as_ref().check(tc).unwrap();
        let mut arguments = vec![];
        
        let ty = callee.get_ty();
        let Type::Function(fn_ty) = ty else {
            tc.errs.push(TypeError::new(
                crate::error::TypeErrorKind::NotCallable {
                    location: self.callee.span(),
                },
                tc.source_file.preparsed,
            ));

            // probably typecheck the arguments
            return Ok(Call {
                callee: Box::new(callee),
                arguments,
            });
        };

        Ok(Call {
            callee: Box::new(callee),
            arguments,
        })
    }
}
