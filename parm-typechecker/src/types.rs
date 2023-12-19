use parm_ast::prelude::*;
macro_rules!  numeric_type {
    ($($name:ident),*) => {
        $(
            #[allow(dead_code)]
            #[derive(Debug,Clone,PartialEq)]
            pub struct $name {
            }

            impl $name {
                #[allow(clippy::new_without_default)]
                pub fn new() -> Self {
                    Self {}
                }
                pub fn name() -> &'static str {
                    lower_stringify!($name)
                }
            }
        )*

        #[derive(Debug,Clone,PartialEq)]
        pub enum Numeric {
            $(
                $name($name),
            )*
        }
    }
}

numeric_type!(U8, U16, U32, U64, U128, I8, I16, I32, I64, I128, F32, F64, F128);

#[derive(Debug, Clone, PartialEq)]
pub enum Boolean {
    True,
    False,
}
#[derive(Debug, Clone, PartialEq)]
pub struct String {}
#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Numeric(Numeric),
    Boolean(Boolean),
    String(String),
}

impl Type {
    pub fn numeric(type_expr: &TypeExpression) -> Self {
        let path = &type_expr.path;
        let path = &path.segments.last;

        let ident = path.as_ref().unwrap();
        match ident.ident.lexeme {
            "u8" => Type::Numeric(Numeric::U8(U8::new())),
            "u16" => Type::Numeric(Numeric::U16(U16::new())),
            "u32" => Type::Numeric(Numeric::U32(U32::new())),
            "u64" => Type::Numeric(Numeric::U64(U64::new())),
            "u128" => Type::Numeric(Numeric::U128(U128::new())),
            "i8" => Type::Numeric(Numeric::I8(I8::new())),
            "i16" => Type::Numeric(Numeric::I16(I16::new())),
            "i32" => Type::Numeric(Numeric::I32(I32::new())),
            "i64" => Type::Numeric(Numeric::I64(I64::new())),
            "i128" => Type::Numeric(Numeric::I128(I128::new())),
            lexeme => panic!("Unknown numeric type: {lexeme}"),
        }
    }
}
