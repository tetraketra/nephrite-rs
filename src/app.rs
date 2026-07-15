use anyhow::Result;
use winit::event_loop::ActiveEventLoop;

mod consts;
mod errors;
mod exttraits;
mod vulkan;
mod window;
mod wrapper;

pub use errors::{AppError, AppResult};
pub use wrapper::AppWrapper;

mod sealed {
    pub trait Sealed {}
}

pub trait State: sealed::Sealed {}

#[derive(Default)]
pub struct Uninitialized;
impl sealed::Sealed for Uninitialized {}
impl State for Uninitialized {}

#[derive(Default)]
pub struct Initialized;
impl sealed::Sealed for Initialized {}
impl State for Initialized {}

#[derive(Default)]
pub struct App<WS: State, VS: State, WD = (), VD = ()> {
    pub window: window::Window<WS, WD>,
    pub vulkan: vulkan::Vulkan<VS, VD>,
}

impl App<Uninitialized, Uninitialized, (), ()> {
    pub fn initialize(
        self,
        event_loop: &ActiveEventLoop,
    ) -> Result<App<Initialized, Initialized, window::WindowData, vulkan::VulkanData>> {
        let window = window::Window::initialize(event_loop)?;
        let vulkan = vulkan::Vulkan::initialize(&window)?;

        Ok(App { window, vulkan })
    }
}
