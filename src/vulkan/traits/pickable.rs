use std::{ffi::CStr, os::raw::c_void};

use anyhow::Result;
use vulkanalia::{
    Instance,
    vk::{self, InstanceV1_0},
};

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

extern "system" fn debug_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    type_: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    let data = unsafe { *data };
    let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();

    if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
        log::error!("({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
        log::warn!("({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
        log::debug!("({:?}) {}", type_, message);
    } else {
        log::trace!("({:?}) {}", type_, message);
    }

    vk::FALSE
}
