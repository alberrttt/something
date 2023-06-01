mod var_decl {
    use something_frontend::VariableDeclaration;

    use crate::{
        context::block::Context,
        traits::ResolveType,
        types::{primitives::Primitive, sig::TypeSig},
    };
    impl ResolveType for VariableDeclaration {
        type Context = Context;

        fn resolve(&self, ctx: &mut Self::Context) -> TypeSig {
            let sig = match self.type_annotation {
                Some(_) => resolve_type_annotated(self, ctx),
                None => resolve_type_inferred(self, ctx),
            };
            ctx.set(&self.name, sig.clone());

            sig
        }
    }
    fn resolve_type_annotated(var_decl: &VariableDeclaration, ctx: &mut Context) -> TypeSig {
        let expr_type = var_decl.value.resolve(ctx);
        let annotation_type = {
            let tmp: Primitive = {
                let (_, ty) = var_decl.type_annotation.as_ref().unwrap();
                ty.into()
            };
            tmp.into()
        };
        if expr_type == annotation_type {
            expr_type
        } else {
            panic!("Type mismatch")
        }
    }

    fn resolve_type_inferred(var_decl: &VariableDeclaration, ctx: &mut Context) -> TypeSig {
        var_decl.value.resolve(ctx)
    }
}
mod expression {
    use something_frontend::{Binary, Expression, Ident, Literal, Operator};

    use crate::{
        context::{block::BlockContext, Context},
        traits::ResolveType,
        types::{primitives::Primitive, sig::TypeSig},
    };

    impl ResolveType for Expression {
        type Context = Context;

        fn resolve(&self, ctx: &mut Self::Context) -> TypeSig {
            match self {
                Expression::Binary(bin) => bin.resolve(ctx),
                Expression::Lit(lit) => lit.resolve(&mut ()),
                Expression::Ident(ident) => ident.resolve(ctx),
                Expression::Call(call) => {
                    let sig = ctx.get(&call.ident).unwrap();
                    match sig {
                        TypeSig::Primitive(_) => panic!("Cannot call a primitive"),
                        TypeSig::Fn(fn_sig) => {
                            let exprs = call
                                .args
                                .iter()
                                .map(|(expr, _)| expr.resolve(ctx))
                                .collect::<Vec<_>>();

                            if fn_sig.0.eq(&exprs) {
                                *fn_sig.1
                            } else {
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

        fn resolve(&self, ctx: &mut Self::Context) -> TypeSig {
            ctx.get(self).unwrap()
        }
    }
    impl ResolveType for Literal {
        type Context = ();

        fn resolve(&self, ctx: &mut Self::Context) -> TypeSig {
            use something_frontend_tokenizer::lit::lit_impl;
            match self.inner {
                lit_impl::Inner::String(_) => Primitive::String.into(),
                lit_impl::Inner::Number(_) => Primitive::Number.into(),
                lit_impl::Inner::Boolean(_) => Primitive::Boolean.into(),
            }
        }
    }
    impl ResolveType for Binary {
        type Context = Context;

        fn resolve(&self, ctx: &mut Self::Context) -> TypeSig {
            match self.operator {
                Operator::Plus | Operator::Minus | Operator::Multiply | Operator::Divide => {
                    //  let's give lhs the precedence
                    // todo: make it so boolean and void types cannot be added
                    let lhs_type = self.left.resolve(ctx);
                    let rhs_type = self.right.resolve(ctx);
                    if lhs_type == rhs_type {
                        lhs_type
                    } else {
                        panic!("Type mismatch")
                    }
                }
                Operator::EqualEqual | Operator::NotEqual => {
                    let lhs_type = self.left.resolve(ctx);
                    let rhs_type = self.right.resolve(ctx);
                    if lhs_type == rhs_type {
                        TypeSig::Primitive(Primitive::Boolean)
                    } else {
                        panic!("Type mismatch")
                    }
                }
                _ => todo!(),
            }
        }
    }
}
