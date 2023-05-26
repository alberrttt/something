use crate::prelude::*;
use something_frontend::{Binary, Expression, Operator};
use std::rc::Rc;

impl TypeCheck<&BlockScope, Type> for Ident {
    fn type_check(&self, with: &BlockScope) -> Type {
        with.get_var(self)
            .unwrap_or_else(|| {
                dbg!(with);

                panic!("variable `{}` not found", self);
            })
            .clone()
    }
}
impl TypeCheck<&mut BlockScope, Type> for &Rc<Ident> {
    fn type_check(&self, with: &mut BlockScope) -> Type {
        with.get_var(self).unwrap().clone()
    }
}
impl TypeCheck<(), Type> for Expression {
    fn type_check(&self, _with: ()) -> Type {
        match self {
            Expression::Binary(binary) => binary.type_check(()),
            Expression::Lit(literal) => literal.type_check(()),
            Expression::Grouping(grouping) => grouping.type_check(()),
            Expression::Call(_) => todo!(),
            Expression::Ident(_) => todo!(),
            Expression::If(_) => todo!(),
            Expression::Block(_) => todo!(),
        }
    }
}
impl TypeCheck<&BlockScope, Type> for Expression {
    fn type_check(&self, with: &BlockScope) -> Type {
        match self {
            Expression::Ident(ident) => ident.type_check(with),
            _ => Expression::type_check(self, ()),
        }
    }
}
impl TypeCheck<&BlockScope, ()> for Expression {
    fn type_check(&self, with: &BlockScope) {
        match self {
            Expression::Ident(ident) => ident.type_check(with),
            _ => Expression::type_check(self, ()),
        };
    }
}
impl TypeCheck<(), Type> for Literal {
    fn type_check(&self, _with: ()) -> Type {
        use lit_impl::Inner::*;
        match self.inner {
            String(_) => Type::string(),
            Number(_) => Type::number(),
            Boolean(_) => Type::boolean(),
        }
    }
}
impl TypeCheck<(), Type> for Binary {
    fn type_check(&self, _with: ()) -> crate::prelude::Type {
        match self.operator {
            Operator::Plus
            | Operator::Minus
            | Operator::Multiply
            | Operator::Divide
            | Operator::PlusEqual
            | Operator::MinusEqual
            | Operator::DivideEqual
            | Operator::MultiplyEqual => Type::number(),

            Operator::EqualEqual
            | Operator::BangEqual
            | Operator::GreaterEqual
            | Operator::LessEqual
            | Operator::Less
            | Operator::Greater => Type::boolean(),

            Operator::Equal => todo!("assignment"),
        }
    }
}

impl TypeCheck<(), Result<Type, TypeError>> for Ident {
    fn type_check(&self, _with: ()) -> Result<Type, TypeError> {
        self.try_into()
    }
}

impl TryFrom<&Ident> for Type {
    type Error = TypeError;

    fn try_from(value: &Ident) -> Result<Self, Self::Error> {
        match value.name.as_str() {
            "number" => Ok(Type::number()),
            "string" => Ok(Type::string()),
            "boolean" => Ok(Type::boolean()),
            _ => Err(TypeError::IncorrectTypeName {
                expected: "number, string or boolean",
                found: value.name.clone(),
            }),
        }
    }
}
