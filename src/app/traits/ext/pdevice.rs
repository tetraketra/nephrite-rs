use anyhow::Result;
use vulkanalia::{
    Instance,
    vk::{self, InstanceV1_0},
};

use crate::app::traits::ext::qfamily::QFamilyExt;

#[derive(Copy, Clone, Debug)]
pub struct QueueFamilyIndices {
    graphics:   u32,
    compute:    u32,
    transfer:   u32,
    sparsebind: u32,
}

pub trait PDeviceExt {
    fn get_supported(
        self,
        instance: &Instance,
    ) -> Result<QueueFamilyIndices>;
}

impl PDeviceExt for vk::PhysicalDevice {
    fn get_supported(
        self,
        instance: &Instance,
    ) -> Result<QueueFamilyIndices> {
        let properties = unsafe { instance.get_physical_device_properties(self) };
        if properties.api_version <= vk::make_version(1, 4, 0) {
            return Err(anyhow::anyhow!("Does not support Vulkan v1.4"));
        }

        let features = unsafe { instance.get_physical_device_features(self) };
        if features.geometry_shader != vk::TRUE {
            return Err(anyhow::anyhow!("Does not support geometry shaders"));
        }

        let qfprops = unsafe { instance.get_physical_device_queue_family_properties(self) };
        Ok(QueueFamilyIndices {
            graphics:   qfprops.get_flag(vk::QueueFlags::GRAPHICS)?,
            compute:    qfprops.get_flag(vk::QueueFlags::COMPUTE)?,
            transfer:   qfprops.get_flag(vk::QueueFlags::TRANSFER)?,
            sparsebind: qfprops.get_flag(vk::QueueFlags::SPARSE_BINDING)?,
        })
    }
}
