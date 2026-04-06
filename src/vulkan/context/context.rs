use anyhow::{Context as _Ctx, Result};
use vulkanalia::{
    loader::{LIBRARY, LibloadingLoader},
    prelude::v1_4::*,
    vk::ExtDebugUtilsExtensionInstanceCommands,
};
use winit::window::Window;

use crate::vulkan::{
    consts,
    context::traits::{Newable, Pickable},
};

#[derive(Clone, Debug, Default)]
pub struct ContextData {
    pub messenger:       vk::DebugUtilsMessengerEXT,
    pub physical_device: vk::PhysicalDevice,
}

#[derive(Clone, Debug)]
pub struct Context {
    entry:    Entry,
    instance: Instance,
    data:     ContextData,
}

impl Context {
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY).with_context(|| "Failed to create loader.")?;
        let entry = Entry::new(loader)
            .map_err(|b| anyhow::anyhow!("{}", b))
            .with_context(|| "Failed to create Vulkan entrypoint.")?;
        let instance = Instance::new(window, &entry).with_context(|| "Failed to create Vulkan instance.")?;
        let mut data = ContextData::default();

        instance.pick_first_physical_device(&mut data)?;

        Ok(Self { entry, instance, data })
    }

    pub unsafe fn render(
        &mut self,
        window: &Window,
    ) -> Result<()> {
        Ok(())
    }

    pub unsafe fn destroy(&mut self) {
        if consts::VALIDATION_ENABLED {
            self.instance
                .destroy_debug_utils_messenger_ext(self.data.messenger, None);
        }

        self.instance.destroy_instance(None);
    }
}
