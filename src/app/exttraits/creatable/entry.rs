use vulkanalia::{
    Entry,
    loader::{LIBRARY, LibloadingLoader},
};

use crate::app::{AppResult, errors::LoadingError, exttraits::Creatable};

impl<'a> Creatable<'a> for Entry {
    type Args = ();

    fn create(_: Self::Args) -> AppResult<Self> {
        let loader = unsafe { LibloadingLoader::new(LIBRARY) }.map_err(|_| {
            LoadingError::Libloading(format!("failed to load LibloadingLoader from: {}", LIBRARY))
        })?;

        let entry = unsafe { Entry::new(loader) }.map_err(LoadingError::VulkanaliaBoxedDyn)?;

        Ok(entry)
    }
}
