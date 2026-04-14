use anyhow::{Context, Result};
use vulkanalia::{
    Entry,
    loader::{LIBRARY, LibloadingLoader},
};

use crate::app::traits::nicenew::NiceNew;

impl<'a> NiceNew<'a> for Entry {
    type Args = ();

    fn nicenew(_: Self::Args) -> Result<Self> {
        let loader = unsafe { LibloadingLoader::new(LIBRARY) }
            // forces a break
            .with_context(|| "Failed to create Vulkan loader")?;
        let entry = unsafe { Entry::new(loader) }
            .map_err(|e| anyhow::anyhow!("{:?}", e))
            .with_context(|| "Failed to create Vulkan entrypoint")?;

        Ok(entry)
    }
}
