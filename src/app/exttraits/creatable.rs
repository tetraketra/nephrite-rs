mod entry;
mod instance;

#[allow(unused_imports)] // `Entry` has unit associated `Args` type.
pub use entry::*;
pub use instance::*;

pub use crate::app::AppResult;

pub trait Creatable<'a>: Sized {
    type Args;
    type Ret = Self;

    fn create(args: Self::Args) -> AppResult<Self::Ret>;
}
