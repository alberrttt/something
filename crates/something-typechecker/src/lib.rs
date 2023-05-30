mod error;
mod file;
mod fn_ctx;
mod primitives;
mod scope;
mod traits;
pub mod prelude {
    pub use crate::error::TypeError;
    pub use crate::fn_ctx::FnCtx;
    pub use crate::primitives::Type;
    pub use crate::scope::block::BlockScope;
}
