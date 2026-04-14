use anyhow::Result;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

use crate::app::{App, Initialized, Uninitialized, vulkan::VulkanData, window::WindowData};

pub enum AppWrapper {
    Uninitialized(App<Uninitialized, Uninitialized>),
    Initialized(App<Initialized, Initialized, WindowData, VulkanData>),
    Transitioning,
}

impl Default for AppWrapper {
    fn default() -> Self {
        Self::Uninitialized(App::default())
    }
}

impl AppWrapper {
    pub fn run_default() -> Result<()> {
        let mut app = Self::default();
        let event_loop = EventLoop::new()?;
        event_loop.run_app(&mut app).map_err(|e| anyhow::anyhow!(e))
    }
}

impl ApplicationHandler for AppWrapper {
    fn exiting(
        &mut self,
        _event_loop: &ActiveEventLoop,
    ) {
        if let AppWrapper::Initialized(app) = std::mem::replace(self, AppWrapper::Transitioning) {
            app.vulkan.deinitialize();
            *self = AppWrapper::Uninitialized(App::default());
            log::info!("AppWrapper successfully deinitialized.");
        }
    }

    fn resumed(
        &mut self,
        event_loop: &ActiveEventLoop,
    ) {
        if let AppWrapper::Uninitialized(app) = std::mem::replace(self, AppWrapper::Transitioning) {
            match app.initialize(event_loop) {
                Ok(initialized_app) => {
                    *self = AppWrapper::Initialized(initialized_app);
                    log::info!("AppWrapper successfully (re)initialized.");
                }
                Err(e) => {
                    log::error!("AppWrapper failed to (re)initialize: {:#}", e);
                    event_loop.exit();
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        if let AppWrapper::Initialized(app) = self {
            match event {
                WindowEvent::RedrawRequested => {
                    let _ = app.vulkan.render(&app.window);
                    app.window.data.window.request_redraw();
                }
                WindowEvent::CloseRequested => {
                    log::info!("AppWrapper close requested; exiting.");
                    event_loop.exit();
                }
                _ => (),
            }
        }
    }
}
