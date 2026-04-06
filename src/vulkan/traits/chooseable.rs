use anyhow::Result;
use thiserror::Error;
use vulkanalia::{
    Instance,
    vk::{self, InstanceV1_0, version_major, version_minor},
};

#[derive(Debug, Error)]
#[error("Missing {0}.")]
pub struct SuitabilityError(pub &'static str);

pub trait Chooseable {
    unsafe fn supports_requirements(
        self,
        instance: &Instance,
    ) -> Result<()>;
}

impl Chooseable for vk::PhysicalDevice {
    unsafe fn supports_requirements(
        self,
        instance: &Instance,
    ) -> Result<()> {
        let qf_properties = instance.get_physical_device_queue_family_properties(self);

        let graphics = qf_properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);

        if graphics.is_none() {
            return Err(anyhow::anyhow!("Does not support required queue families"));
        }

        let properties = instance.get_physical_device_properties(self);
        if version_major(properties.api_version) != 1 || version_minor(properties.api_version) < 4 {
            return Err(anyhow::anyhow!(SuitabilityError("Does not support Vulkan v1.3")));
        }

        let features = instance.get_physical_device_features(self);
        if features.geometry_shader != vk::TRUE {
            return Err(anyhow::anyhow!(SuitabilityError("Does not support geometry shaders")));
        }

        Ok(())
    }
}
