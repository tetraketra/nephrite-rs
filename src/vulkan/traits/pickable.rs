use anyhow::Result;
use vulkanalia::{Instance, vk::InstanceV1_0};

use crate::vulkan::{context::ContextData, traits::Chooseable};

pub trait Pickable {
    unsafe fn pick_first_physical_device(
        &self,
        data: &mut ContextData,
    ) -> Result<()>;
}

impl Pickable for Instance {
    unsafe fn pick_first_physical_device(
        &self,
        data: &mut ContextData,
    ) -> Result<()> {
        for physical_device in self.enumerate_physical_devices()? {
            let properties = self.get_physical_device_properties(physical_device);

            if let Err(error) = physical_device.supports_requirements(self) {
                log::warn!("Skipping phyisical device (`{}`): {}", properties.device_name, error);
            } else {
                log::info!("Selected physical device (`{}`)", properties.device_name);
                data.physical_device = physical_device;
                return Ok(());
            }
        }

        Err(anyhow::anyhow!("Failed to find suitable physical device"))
    }
}
