use std::marker::PhantomData;

use vulkanalia::{prelude::v1_4::*, vk::ExtDebugUtilsExtensionInstanceCommands};

use crate::app::{
    AppError, AppResult, Initialized, State, Uninitialized, consts,
    exttraits::{Creatable, InstanceArgs, PDeviceExt, QueueFamilyIndices},
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
    pub fn initialize(
        window: &Window<Initialized, WindowData>
    ) -> AppResult<Vulkan<Initialized, VulkanData>> {
        let entry = Entry::create(())?;

        let (instance, messenger) = Instance::create(InstanceArgs {
            window: &window.data.window,
            entry:  &entry,
        })?;

        let (pdevice, qfamily) = unsafe { instance.enumerate_physical_devices() }?
            .into_iter()
            .find_map(|pd| pd.get_supported(&instance).ok().map(|qf| (pd, qf)))
            .ok_or_else(|| AppError::Hardware("no supported physical devices".into()))?;

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
    ) -> AppResult<()> {
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
