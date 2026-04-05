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
    Initialized { window: Window, context: Context },
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
        let window = event_loop
            .create_window(Window::default_attributes().with_title("Nephrite Project"))
            .expect("Failed to create window.");

        let context = unsafe { Context::create(&window).expect("Failed to create Vulkan context.") };

        self.state = AppState::Initialized { window, context };
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
