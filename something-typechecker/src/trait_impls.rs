mod var_decl {
    use something_frontend::VariableDeclaration;

    use crate::{
        context::block::Context,
        error::TypeError,
        traits::ResolveType,
        types::{primitives::Primitive, sig::TypeSig},
    };
    impl ResolveType for VariableDeclaration {
        type Context = Context;

        fn resolve(&self, ctx: &mut Self::Context) -> Result<TypeSig, TypeError> {
            let sig = match self.type_annotation {
                Some(_) => resolve_type_annotated(self, ctx)?,
                None => resolve_type_inferred(self, ctx)?,
            };
            ctx.set(&self.name, sig.clone());

            Ok(sig)
        }
    }
    fn resolve_type_annotated(
        var_decl: &VariableDeclaration,
        ctx: &mut Context,
    ) -> Result<TypeSig, TypeError> {
        let expr_type = var_decl.expression.resolve(ctx)?;
        let annotation_type = {
            let tmp: Primitive = {
                let (_, ty) = var_decl.type_annotation.as_ref().unwrap();
                ty.into()
            };
            tmp.into()
        };
        if expr_type == annotation_type {
            Ok(expr_type)
        } else {
            Err(TypeError::ExpectedType {
                expected: annotation_type,
                found: expr_type,
            })
        }
    }

    fn resolve_type_inferred(
        var_decl: &VariableDeclaration,
        ctx: &mut Context,
    ) -> Result<TypeSig, TypeError> {
        var_decl.expression.resolve(ctx)
    }
}
mod expression {
    use something_frontend::{Binary, Expression, Ident, Literal, OperatorKind};

    use crate::{
        context::{Context},
        error::{self, TypeError},
        traits::ResolveType,
        types::{self, primitives::Primitive, sig::TypeSig},
    };

    impl ResolveType for Expression {
        type Context = Context;

        fn resolve(
            &self,
            ctx: &mut Self::Context,
        ) -> Result<types::sig::TypeSig, error::TypeError> {
            match self {
                Expression::Binary(bin) => bin.resolve(ctx),
                Expression::Lit(lit) => lit.resolve(&mut ()),
                Expression::Ident(ident) => ident.resolve(ctx),
                Expression::Call(call) => {
                    let call_sig = ctx.get(&call.ident).unwrap();
                    match call_sig {
                        TypeSig::Primitive(_) => panic!("Cannot call a primitive"),
                        TypeSig::Fn(fn_sig) => {
                            let argument_sig: Vec<TypeSig> = call
                                .args
                                .iter()
                                .map(|(expr, _)| expr.resolve(ctx))
                                .collect::<Result<Vec<_>, TypeError>>()?;

                            if fn_sig.0.eq(&argument_sig) {
                                Ok(*fn_sig.1)
                            } else {
                                // parameter is the declared
                                // argument is the actual/passed in
                                for (parameter_sig, argument_sig) in
                                    fn_sig.0.iter().zip(argument_sig.iter())
                                {
                                    if parameter_sig != argument_sig {
                                        return Err(TypeError::ExpectedType {
                                            expected: parameter_sig.clone(),
                                            found: argument_sig.clone(),
                                        });
                                    }
                                }
                                panic!("Type mismatch")
                            }
                        }
                    }
                }
                _ => todo!(),
            }
        }
    }
    impl ResolveType for Ident {
        type Context = Context;

        fn resolve(&self, ctx: &mut Self::Context) -> Result<TypeSig, TypeError> {
            match ctx.get(self) {
                Some(tmp) => Ok(tmp),
                None => Err(TypeError::IdentOutOfScope(self.clone())),
            }
        }
    }
    impl ResolveType for Literal {
        type Context = ();

        fn resolve(&self, _ctx: &mut Self::Context) -> Result<TypeSig, TypeError> {
            use something_ast::tokenizer::lit::lit_impl;
            Ok(match self.inner {
                lit_impl::Inner::String(_) => Primitive::String.into(),
                lit_impl::Inner::Number(_) => Primitive::Number.into(),
                lit_impl::Inner::Boolean(_) => Primitive::Boolean.into(),
            })
        }
    }
    impl ResolveType for Binary {
        type Context = Context;

        fn resolve(&self, ctx: &mut Self::Context) -> Result<TypeSig, TypeError> {
            match self.operator.kind {
                OperatorKind::Plus
                | OperatorKind::Minus
                | OperatorKind::Multiply
                | OperatorKind::Divide => {
                    //  let's give lhs the precedence
                    // todo: make it so boolean and void types cannot be added
                    let lhs_type = self.left.resolve(ctx)?;
                    let rhs_type = self.right.resolve(ctx)?;
                    if lhs_type == rhs_type {
                        Ok(lhs_type)
                    } else {
                        Err(TypeError::MismatchedTypes {
                            expected: lhs_type,
                            found: rhs_type,
                        })
                    }
                }
                OperatorKind::EqualEqual | OperatorKind::NotEqual => {
                    let lhs_type = self.left.resolve(ctx)?;
                    let rhs_type = self.right.resolve(ctx)?;
                    if lhs_type == rhs_type {
                        Ok(TypeSig::Primitive(Primitive::Boolean))
                    } else {
                        Err(TypeError::MismatchedTypes {
                            expected: lhs_type,
                            found: rhs_type,
                        })
                    }
                }
                _ => todo!(),
            }
        }
    }
}
