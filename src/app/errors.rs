use thiserror::Error;
use vulkanalia::{loader::LoaderError, vk::ErrorCode};
use winit::{error::OsError, window::BadIcon};

#[derive(Error, Debug)]
pub enum ImageError {
    #[error("generic image [de|en]coding: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("winit BadIcon: {0}")]
    BadIconError(#[from] BadIcon),
}

#[derive(Error, Debug)]
pub enum LoadingError {
    #[error("LibLoading opaque error: {0}")]
    Libloading(String),

    #[error("Vulkanalia dynamic LoaderError: {0}")]
    VulkanaliaBoxedDyn(Box<dyn LoaderError>),

    #[error("image ImageError: {0}")]
    Image(#[from] ImageError),
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Nephrite loading error: {0}")]
    Loading(#[from] LoadingError),

    #[error("Vuklanalia opaque Vulkan error code: {0}")]
    VkCode(#[from] ErrorCode),

    #[error("Nephrite validation error: {0}")]
    Validation(String),

    #[error("Nephrite hardware error: {0}")]
    Hardware(String),

    #[error("winit generic OsError: {0}")]
    WinitOs(#[from] OsError),
}

pub type AppResult<T> = Result<T, AppError>;
