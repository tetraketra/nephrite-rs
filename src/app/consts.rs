use vulkanalia::{Version, vk};

pub const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
pub const VALIDATION_LAYER: vk::ExtensionName = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");
pub const MACOS_PORTABILITY_VERSION: Version = Version::new(1, 3, 216);
