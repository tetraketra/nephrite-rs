use anyhow::{Context as _Ctx, Result};
use vulkanalia::{
    Version,
    loader::{LIBRARY, LibloadingLoader},
    prelude::v1_4::*,
    window as vk_window,
};
use winit::window::Window;

const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct Context {
    entry: Entry,
    instance: Instance,
}

#[allow(unsafe_op_in_unsafe_fn)]
impl Context {
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY).with_context(|| "Failed to create Vulkan loader.")?;
        let entry = Entry::new(loader)
            .map_err(|b| anyhow::anyhow!("{}", b))
            .with_context(|| "Failed to create Vulkan entrypoint.")?;
        let instance = Context::create_instance(window, &entry)?;

        Ok(Self { entry, instance })
    }

    #[allow(unused)]
    pub unsafe fn render(
        &mut self,
        window: &Window,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(unused)]
    pub unsafe fn destroy(&mut self) {
        self.instance.destroy_instance(None);
    }

    unsafe fn create_instance(
        window: &Window,
        entry: &Entry,
    ) -> Result<Instance> {
        let application_info = vk::ApplicationInfo::builder()
            .application_name(b"Nephrite Project\0")
            .application_version(vk::make_version(1, 4, 0))
            .engine_name(b"Nephrite\0")
            .engine_version(vk::make_version(1, 0, 0))
            .api_version(vk::make_version(1, 4, 0));

        let mut extensions = vk_window::get_required_instance_extensions(window)
            .iter()
            .map(|e| e.as_ptr())
            .collect::<Vec<_>>();

        let flags = if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
            log::info!("Enabling extensions for MacOS portability.");
            extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr());
            extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
            vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        } else {
            vk::InstanceCreateFlags::empty()
        };

        let info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_extension_names(&extensions)
            .flags(flags);

        Ok(entry.create_instance(&info, None)?)
    }
}
