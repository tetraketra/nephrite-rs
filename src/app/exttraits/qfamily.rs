use vulkanalia::vk;
use vulkanalia_sys::QueueFamilyProperties;

use crate::app::{AppError, AppResult};

pub trait QFamilyExt {
    fn get_flag(
        &self,
        flag: vk::QueueFlags,
        name: &str,
    ) -> AppResult<u32>;
}

impl QFamilyExt for Vec<QueueFamilyProperties> {
    fn get_flag(
        &self,
        flag: vk::QueueFlags,
        name: &str,
    ) -> AppResult<u32> {
        self.iter()
            .position(|p| p.queue_flags.contains(flag))
            .map(|i| i as u32)
            .ok_or(AppError::Hardware(format!(
                "doesn't support required queue family: {:#b} ({})",
                flag.bits(),
                name
            )))
    }
}
