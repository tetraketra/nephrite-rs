use anyhow::{Context as _Ctx, Result};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::vulkan::Context;

#[derive(Debug)]
enum AppState {
    Uninitialized,
    Initialized {
        window:  Window,
        context: Context,
    },
}

#[derive(Debug)]
pub struct App {
    state: AppState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: AppState::Uninitialized,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(
        &mut self,
        event_loop: &ActiveEventLoop,
    ) {
        if let AppState::Uninitialized = self.state {
            let window = match event_loop.create_window(Window::default_attributes()) {
                Ok(w) => w,
                Err(e) => {
                    log::error!("Failed to create window: {:#}", e);
                    event_loop.exit();
                    return;
                }
            };

            match unsafe { Context::create(&window) } {
                Ok(context) => self.state = AppState::Initialized { window, context },
                Err(e) => {
                    log::error!("Application failed to start: {:#}", e);
                    event_loop.exit();
                }
            }
        };
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        match &self.state {
            AppState::Initialized { window, .. } => match event {
                WindowEvent::CloseRequested => {
                    log::info!("Window was closed via close button; exiting.");
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => window.request_redraw(),
                _ => (),
            },
            _ => {}
        }
    }
}
