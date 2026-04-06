use image::GenericImageView;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Icon, Window, WindowId},
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
    fn exiting(
        &mut self,
        _event_loop: &ActiveEventLoop,
    ) {
        if let AppState::Initialized { context, .. } = &mut self.state {
            unsafe { context.destroy() };
        }
    }

    fn resumed(
        &mut self,
        event_loop: &ActiveEventLoop,
    ) {
        if let AppState::Uninitialized = self.state {
            let icon_bytes = include_bytes!("../../assets/icon_jade_1.png");
            let icon_mem = match image::load_from_memory(icon_bytes) {
                Ok(i) => i,
                Err(e) => {
                    log::error!("Failed to load icon from memory: {:#}", e);
                    event_loop.exit();
                    return;
                }
            };
            let (width, height) = icon_mem.dimensions();
            let icon_rgba = icon_mem.to_rgba8().into_raw();
            let icon = match Icon::from_rgba(icon_rgba, width, height) {
                Ok(i) => Some(i),
                Err(e) => {
                    log::error!("Failed to load icon from memory: {:#}", e);
                    event_loop.exit();
                    return;
                }
            };

            let window = match event_loop.create_window(
                Window::default_attributes()
                    .with_title("Nephrite")
                    .with_window_icon(icon),
            ) {
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
                WindowEvent::RedrawRequested => window.request_redraw(),
                WindowEvent::CloseRequested => {
                    log::info!("Window was closed via close button; exiting.");
                    event_loop.exit();
                }
                _ => (),
            },
            _ => {}
        }
    }
}
