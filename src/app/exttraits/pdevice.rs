use tap::Pipe;
use vulkanalia::{
    Instance,
    vk::{self, InstanceV1_0},
};

use crate::app::{AppError, AppResult, exttraits::qfamily::QFamilyExt};

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
    ) -> AppResult<QueueFamilyIndices>;
}

impl PDeviceExt for vk::PhysicalDevice {
    fn get_supported(
        self,
        instance: &Instance,
    ) -> AppResult<QueueFamilyIndices> {
        let maj = 1;
        let min = 4;
        let pat = 0;
        let vk_version = vk::make_version(maj, min, pat);

        let _properties = unsafe { instance.get_physical_device_properties(self) }.pipe(|props| {
            if props.api_version <= vk_version {
                Err(AppError::Hardware(format!(
                    "doesn't support Vuklan {}.{}.{}",
                    maj, min, pat
                )))
            } else {
                Ok(props)
            }
        })?;

        let _features = unsafe { instance.get_physical_device_features(self) }.pipe(|feats| {
            if feats.geometry_shader != vk::TRUE {
                Err(AppError::Hardware("doesn't support geometry shaders".into()))
            } else {
                Ok(feats)
            }
        })?;

        let qfprops = unsafe { instance.get_physical_device_queue_family_properties(self) };

        Ok(QueueFamilyIndices {
            graphics:   qfprops.get_flag(vk::QueueFlags::GRAPHICS, "GRAPHICS")?,
            compute:    qfprops.get_flag(vk::QueueFlags::COMPUTE, "COMPUTE")?,
            transfer:   qfprops.get_flag(vk::QueueFlags::TRANSFER, "TRANSFER")?,
            sparsebind: qfprops.get_flag(vk::QueueFlags::SPARSE_BINDING, "SPARSE_BINDING")?,
        })
    }
}
