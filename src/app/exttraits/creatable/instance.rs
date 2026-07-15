use std::{collections::HashSet, ffi::CStr, os::raw::c_void};

use tap::Pipe;
use vulkanalia::{
    Instance,
    prelude::v1_4::*,
    vk::{self, ExtDebugUtilsExtensionInstanceCommands},
    window as vk_window,
};
use vulkanalia_sys::DebugUtilsMessengerEXT;
use winit::window::Window;

use crate::app::{AppError, AppResult, consts, exttraits::Creatable};

pub struct InstanceArgs<'a> {
    pub window: &'a Window,
    pub entry:  &'a Entry,
}

impl<'a> Creatable<'a> for Instance {
    type Args = InstanceArgs<'a>;
    type Ret = (Self, DebugUtilsMessengerEXT);

    fn create(args: Self::Args) -> AppResult<Self::Ret> {
        let application_info = vk::ApplicationInfo::builder()
            .application_name(b"Nephrite\0")
            .application_version(vk::make_version(1, 4, 0))
            .engine_name(b"Nephrite\0")
            .engine_version(vk::make_version(1, 0, 0))
            .api_version(vk::make_version(1, 4, 0));

        let _available_layers = unsafe { args.entry.enumerate_instance_layer_properties() }?
            .iter()
            .map(|l| l.layer_name)
            .collect::<HashSet<_>>()
            .pipe(|al| {
                if consts::VALIDATION_ENABLED && !al.contains(&consts::VALIDATION_LAYER) {
                    Err(AppError::Validation(
                        "Validation layer requested but not supported".into(),
                    ))
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

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_extension_names(&extensions)
            .enabled_layer_names(&layers)
            .flags(flags)
            .pipe(|create_info| {
                if consts::VALIDATION_ENABLED {
                    create_info.push_next(&mut debug_info)
                } else {
                    create_info
                }
            });

        let instance = unsafe { args.entry.create_instance(&create_info, None) }?;

        let messenger = unsafe { instance.create_debug_utils_messenger_ext(&debug_info, None) }?;

        Ok((instance, messenger))
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
