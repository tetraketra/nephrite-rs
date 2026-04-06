use std::{collections::HashSet, ffi::CStr, os::raw::c_void};

use anyhow::{Context as _Ctx, Result};
use tap::Pipe;
use vulkanalia::{prelude::v1_4::*, window as vk_window};
use winit::window::Window;

use crate::vulkan::{
    consts,
    context::{context::ContextData, device::ChooseablePhysicalDevice},
};

pub trait Newable {
    fn new(
        window: &Window,
        entry: &Entry,
    ) -> Result<Self>
    where
        Self: Sized;
}

impl Newable for Instance {
    fn new(
        window: &Window,
        entry: &Entry,
    ) -> Result<Instance> {
        let application_info = vk::ApplicationInfo::builder()
            .application_name(b"Nephrite Project\0")
            .application_version(vk::make_version(1, 4, 0))
            .engine_name(b"Nephrite\0")
            .engine_version(vk::make_version(1, 0, 0))
            .api_version(vk::make_version(1, 4, 0));

        let _available_layers = unsafe { entry.enumerate_instance_layer_properties() }
            .with_context(|| "Failed to enumerate instance layer properties")?
            .iter()
            .map(|l| l.layer_name)
            .collect::<HashSet<_>>()
            .pipe(|al| {
                if consts::VALIDATION_ENABLED && !al.contains(&consts::VALIDATION_LAYER) {
                    Err(anyhow::anyhow!("Validation layer requested but not supported"))
                } else {
                    Ok(al)
                }
            })?;

        let layers = if consts::VALIDATION_ENABLED {
            vec![consts::VALIDATION_LAYER.as_ptr()]
        } else {
            Vec::new()
        };

        let is_macos_portable = cfg!(target_os = "macos") && entry.version()? >= consts::MACOS_PORTABILITY_VERSION;
        let macos_extensions = [
            vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr(),
            vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr(),
        ];

        let extensions = vk_window::get_required_instance_extensions(window)
            .iter()
            .map(|e| e.as_ptr())
            .chain(consts::VALIDATION_ENABLED.then_some(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr()))
            .chain(is_macos_portable.then_some(macos_extensions).into_iter().flatten())
            .collect::<Vec<_>>();

        let flags = if is_macos_portable {
            vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        } else {
            vk::InstanceCreateFlags::empty()
        };

        let mut debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .user_callback(Some(debug_callback));

        let info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_extension_names(&extensions)
            .enabled_layer_names(&layers)
            .flags(flags)
            .pipe(|info| {
                if consts::VALIDATION_ENABLED {
                    info.push_next(&mut debug_info)
                } else {
                    info
                }
            });

        let instance = unsafe { entry.create_instance(&info, None) }.with_context(|| "Failed to create instance")?;

        Ok(instance)
    }
}

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
