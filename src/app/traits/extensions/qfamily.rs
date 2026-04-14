use anyhow::Result;
use vulkanalia::vk;
use vulkanalia_sys as vksys;

pub trait QFamilyExt {
    fn get_flag(
        &self,
        flag: vk::QueueFlags,
    ) -> Result<u32>;
}

impl QFamilyExt for Vec<vksys::QueueFamilyProperties> {
    fn get_flag(
        &self,
        flag: vk::QueueFlags,
    ) -> Result<u32> {
        self.iter()
            .position(|p| p.queue_flags.contains(flag))
            .map(|i| i as u32)
            .ok_or(anyhow::anyhow!(
                "Does not support required queue families: {:?}",
                flag
            ))
    }
}
