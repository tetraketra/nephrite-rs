use std::{collections::HashSet, ffi::CStr, os::raw::c_void};

use anyhow::{Context as _Ctx, Result};
use tap::Pipe;
use vulkanalia::{
    Instance,
    prelude::v1_4::*,
    vk::{self, ExtDebugUtilsExtensionInstanceCommands},
    window as vk_window,
};
use vulkanalia_sys::DebugUtilsMessengerEXT;
use winit::window::Window;

use crate::app::{consts, traits::nicenew::NiceNew};

pub struct NNInstance<'a> {
    pub window:    &'a Window,
    pub entry:     &'a Entry,
    pub messenger: &'a mut DebugUtilsMessengerEXT,
}

impl<'a> NiceNew<'a> for Instance {
    type Args = NNInstance<'a>;

    fn nice_new(args: Self::Args) -> Result<Self> {
        let application_info = vk::ApplicationInfo::builder()
            .application_name(b"Nephrite\0")
            .application_version(vk::make_version(1, 4, 0))
            .engine_name(b"Nephrite\0")
            .engine_version(vk::make_version(1, 0, 0))
            .api_version(vk::make_version(1, 4, 0));

        let _available_layers = unsafe { args.entry.enumerate_instance_layer_properties() }
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

        let is_macos_portable =
            cfg!(target_os = "macos") && args.entry.version()? >= consts::MACOS_PORTABILITY_VERSION;
        let macos_extensions = [
            vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr(),
            vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr(),
        ];

        let extensions = vk_window::get_required_instance_extensions(args.window)
            .iter()
            .map(|e| e.as_ptr())
            .chain(consts::VALIDATION_ENABLED.then_some(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr()))
            .chain(
                is_macos_portable
                    .then_some(macos_extensions)
                    .into_iter()
                    .flatten(),
            )
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

        let instance = unsafe { args.entry.create_instance(&info, None) }
            .with_context(|| "Failed to create Vulkan instance")?;

        *args.messenger = unsafe { instance.create_debug_utils_messenger_ext(&debug_info, None) }
            .with_context(|| "Failed to create Vulkan debug messenger")?;

        Ok(instance)
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
