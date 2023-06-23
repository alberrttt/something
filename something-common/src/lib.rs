#![feature(try_trait_v2)]
#[macro_export]
macro_rules! devprintln {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            print!(concat!("[",file!(), ":", line!(), "]: "));
            println!($($arg)*);

        }
    }
}
use std::{convert::Infallible, fmt::Debug, ops::FromResidual};

pub enum Result<T, E> {
    Ok(T),
    Recoverable,
    Err(E),
}

impl<T, E> Debug for Result<T, E>
where
    T: Debug,
    E: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Result::Ok(ok) => write!(f, "Ok({:?})", ok),
            Result::Recoverable => write!(f, "Recoverable"),
            Result::Err(err) => write!(f, "Err({:?})", err),
        }
    }
}
impl<T, E> Clone for Result<T, E>
where
    T: Clone,
    E: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Result::Ok(ok) => Result::Ok(ok.clone()),
            Result::Recoverable => Result::Recoverable,
            Result::Err(err) => Result::Err(err.clone()),
        }
    }
}
impl<T, E> PartialEq for Result<T, E>
where
    T: PartialEq,
    E: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Result::Ok(ok1), Result::Ok(ok2)) => ok1 == ok2,
            (Result::Recoverable, Result::Recoverable) => true,
            (Result::Err(err1), Result::Err(err2)) => err1 == err2,
            _ => false,
        }
    }
}
impl<T, E> Eq for Result<T, E>
where
    T: Eq,
    E: Eq,
{
}
impl<T, E> std::hash::Hash for Result<T, E>
where
    T: std::hash::Hash,
    E: std::hash::Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Result::Ok(ok) => {
                state.write_u8(0);
                ok.hash(state);
            }
            Result::Recoverable => {
                state.write_u8(1);
            }
            Result::Err(err) => {
                state.write_u8(2);
                err.hash(state);
            }
        }
    }
}

impl<T, E> Result<T, E> {
    #[track_caller]
    pub fn unwrap(self) -> T
    where
        T: std::fmt::Debug,
        E: std::error::Error,
    {
        match self {
            Result::Ok(ok) => ok,
            Result::Recoverable => panic!("Attempted to panic on recoverable error"),
            Result::Err(err) => {
                devprintln!("\n{}", err);
                panic!()
            }
        }
    }
    pub fn is_ok(&self) -> bool {
        match self {
            Result::Ok(_) => true,
            Result::Recoverable => false,
            Result::Err(_) => false,
        }
    }
    pub fn is_err(&self) -> bool {
        match self {
            Result::Ok(_) => false,
            Result::Recoverable => false,
            Result::Err(_) => true,
        }
    }
    pub fn is_recoverable(&self) -> bool {
        match self {
            Result::Ok(_) => false,
            Result::Recoverable => true,
            Result::Err(_) => false,
        }
    }
    pub fn ok(self) -> Option<T> {
        match self {
            Result::Ok(ok) => Some(ok),
            Result::Recoverable => None,
            Result::Err(_) => None,
        }
    }
    pub fn err(self) -> Option<E> {
        match self {
            Result::Ok(_) => None,
            Result::Recoverable => None,
            Result::Err(err) => Some(err),
        }
    }
}
impl<T, E> FromResidual<Result<Infallible, E>> for Result<T, E> {
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Result::Recoverable => Self::Recoverable,
            Result::Err(err) => Self::Err(err),
            _ => unreachable!(),
        }
    }
}
use crate::Result::*;
impl<T, E> std::ops::Try for crate::Result<T, E> {
    type Output = T;

    type Residual = crate::Result<Infallible, E>;

    fn from_output(output: Self::Output) -> Self {
        crate::Result::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            Ok(ok) => std::ops::ControlFlow::Continue(ok),
            Err(err) => std::ops::ControlFlow::Break(crate::Result::Err(err)),

            Recoverable => std::ops::ControlFlow::Break(crate::Recoverable),
        }
    }
}
impl<Ok, Err> From<crate::Result<Ok, Err>> for std::result::Result<Ok, Err> {
    fn from(value: crate::Result<Ok, Err>) -> Self {
        match value {
            crate::Result::Ok(ok) => std::result::Result::Ok(ok),
            crate::Result::Recoverable => {
                panic!("Unable to convert recoverable error to std::result::Result")
            }
            crate::Result::Err(err) => std::result::Result::Err(err),
        }
    }
}
