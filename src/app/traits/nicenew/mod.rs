pub mod entry;
pub mod instance;
pub mod prelude;

use anyhow::Result;

pub trait NiceNew<'a> {
    type Args;

    fn nicenew(args: Self::Args) -> Result<Self>
    where
        Self: Sized;
}
