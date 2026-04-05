use std::{collections::HashSet, ffi::CStr, os::raw::c_void};

use anyhow::{Context as _Ctx, Result};
use vulkanalia::{
    Version,
    loader::{LIBRARY, LibloadingLoader},
    prelude::v1_4::*,
    vk::{ApplicationInfo, ExtDebugUtilsExtensionInstanceCommands},
    window as vk_window,
};
use winit::window::Window;

pub const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
pub const VALIDATION_LAYER: vk::ExtensionName = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

pub const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);

#[derive(Clone, Debug, Default)]
pub struct InstanceData {
    pub messenger: vk::DebugUtilsMessengerEXT,
}

#[derive(Clone, Debug)]
pub struct Context {
    entry: Entry,
    instance: Instance,
    data: InstanceData,
}

#[allow(unsafe_op_in_unsafe_fn)]
impl Context {
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY).with_context(|| "Failed to create Vulkan loader.")?;
        let entry = Entry::new(loader)
            .map_err(|b| anyhow::anyhow!("{}", b))
            .with_context(|| "Failed to create Vulkan entrypoint.")?;
        let mut data = InstanceData::default();
        let instance =
            Context::create_instance(window, &entry, &mut data).with_context(|| "Failed to create Vulkan instance.")?;

        Ok(Self { entry, instance, data })
    }

    pub unsafe fn render(
        &mut self,
        window: &Window,
    ) -> Result<()> {
        Ok(())
    }

    pub unsafe fn destroy(&mut self) {
        if VALIDATION_ENABLED {
            self.instance
                .destroy_debug_utils_messenger_ext(self.data.messenger, None);
        }

        self.instance.destroy_instance(None);
    }

    unsafe fn create_instance(
        window: &Window,
        entry: &Entry,
        data: &mut InstanceData,
    ) -> Result<Instance> {
        // App Info
        let application_info = vk::ApplicationInfo::builder()
            .application_name(b"Nephrite Project\0")
            .application_version(vk::make_version(1, 4, 0))
            .engine_name(b"Nephrite\0")
            .engine_version(vk::make_version(1, 0, 0))
            .api_version(vk::make_version(1, 4, 0));

        let available_layers = unsafe {
            entry
                .enumerate_instance_layer_properties()
                .with_context(|| "Failed to enumerate instance layer properties.")?
        }
        .iter()
        .map(|l| l.layer_name)
        .collect::<HashSet<_>>();

        if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER) {
            return Err(anyhow::anyhow!("Validation layer requested but not supported."));
        }

        // Layers
        let layers = if VALIDATION_ENABLED {
            vec![VALIDATION_LAYER.as_ptr()]
        } else {
            Vec::new()
        };

        // Extensions
        let mut extensions = vk_window::get_required_instance_extensions(window)
            .iter()
            .map(|e| e.as_ptr())
            .collect::<Vec<_>>();

        if VALIDATION_ENABLED {
            extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());
        };

        // Flags
        let flags = if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
            log::info!("Enabling extensions for MacOS portability.");
            extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr());
            extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
            vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        } else {
            vk::InstanceCreateFlags::empty()
        };

        // Instance
        let mut info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_extension_names(&extensions)
            .enabled_layer_names(&layers)
            .flags(flags);

        let mut debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .user_callback(Some(Self::debug_callback));

        if VALIDATION_ENABLED {
            info = info.push_next(&mut debug_info);
        }

        let instance = unsafe {
            entry
                .create_instance(&info, None)
                .with_context(|| "Failed to create instance.")?
        };

        Ok(instance)
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
}
