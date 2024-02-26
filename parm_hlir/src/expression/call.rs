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
        for arg in self.arguments.inner.elements() {
            match arg.check(tc) {
                Ok(arg) => {
                    arguments.push(arg);
                }
                Err(err) => arguments.push(Expression::None),
            }
        }

        let ty = callee.get_ty();
        let Type::Function(fn_ty) = ty else {
            if let Type::Unknown { err } = ty {
                if err {
                    return Ok(Call {
                        callee: Box::new(callee),
                        arguments,
                    });
                }
            }
            tc.errs.push(TypeError::new(
                crate::error::TypeErrorKind::NotCallable {
                    location: self.span(),
                },
                tc.source_file.preparsed,
            ));
            return Ok(Call {
                callee: Box::new(callee),
                arguments,
            });
        };

        if fn_ty.params.len() != arguments.len() {
            tc.errs.push(TypeError::new(
                crate::error::TypeErrorKind::IncorrectArgs {
                    expected: fn_ty.params.len() as u8,
                    got: arguments.len() as u8,
                    location: self.arguments.span(),
                },
                tc.source_file.preparsed,
            ));
        }

        for (i, (param, arg)) in fn_ty
            .as_ref()
            .params
            .iter()
            .zip(arguments.iter())
            .enumerate()
        {
            if !param.ty().eq_amb(&arg.get_ty()) {
                tc.errs.push(TypeError::new(
                    crate::error::TypeErrorKind::MismatchedTypes {
                        expected: param.ty(),
                        got: arg.get_ty().clone(),
                        location: self.arguments.inner.elements()[i].span(),
                    },
                    tc.source_file.preparsed,
                ));
            }
        }

        Ok(Call {
            callee: Box::new(callee),
            arguments,
        })
    }
}
