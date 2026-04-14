use std::marker::PhantomData;

use anyhow::{Context as _Ctx, Result};
use vulkanalia::{prelude::v1_4::*, vk::ExtDebugUtilsExtensionInstanceCommands};
use vulkanalia_sys::DebugUtilsMessengerEXT;

use crate::app::{
    Initialized, State, Uninitialized, consts,
    traits::{ext::prelude::*, nicenew::prelude::*},
    window::{Window, WindowData},
};

pub struct VulkanData {
    pub entry:    Entry,
    pub instance: Instance,

    pub qfamily:   QueueFamilyIndices,
    pub pdevice:   vk::PhysicalDevice,
    pub messenger: vk::DebugUtilsMessengerEXT,
}

pub struct Vulkan<S: State, D = ()> {
    pub data: D,
    _marker:  PhantomData<S>,
}

impl<S: State, D: Default> Default for Vulkan<S, D> {
    fn default() -> Self {
        Self {
            data:    D::default(),
            _marker: PhantomData,
        }
    }
}

impl Vulkan<Uninitialized, ()> {
    pub fn initialize(window: &Window<Initialized, WindowData>) -> Result<Vulkan<Initialized, VulkanData>> {
        let mut messenger = DebugUtilsMessengerEXT::default();
        let entry = Entry::nicenew(()).with_context(|| "Failed to create Vulkan entry")?;
        let instance = Instance::nicenew(NNInstance {
            window:    &window.data.window,
            entry:     &entry,
            messenger: &mut messenger,
        })
        .with_context(|| "Failed to create Vulkan instance")?;
        let (pdevice, qfamily) = unsafe { instance.enumerate_physical_devices() }?
            .into_iter()
            .find_map(|pd| pd.get_supported(&instance).ok().map(|qf| (pd, qf)))
            .ok_or_else(|| anyhow::anyhow!("No suitable starter phyiscal device found"))?;

        Ok(Vulkan {
            data: VulkanData {
                entry,
                instance,
                pdevice,
                qfamily,
                messenger,
            },

            _marker: PhantomData,
        })
    }
}

impl Vulkan<Initialized, VulkanData> {
    pub fn render(
        &mut self,
        window: &Window<Initialized, WindowData>,
    ) -> Result<()> {
        // Has access to data.
        Ok(())
    }

    pub fn deinitialize(self) -> Vulkan<Uninitialized> {
        unsafe {
            if consts::VALIDATION_ENABLED {
                self.data
                    .instance
                    .destroy_debug_utils_messenger_ext(self.data.messenger, None);
            }

            self.data.instance.destroy_instance(None);
        }

        Vulkan::default()
    }
}
