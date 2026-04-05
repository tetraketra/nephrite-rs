#![cfg_attr(debug_assertions, allow(unused))]

use anyhow::Result;
use winit::event_loop::{ControlFlow, EventLoop};

mod app;
mod vulkan;

use app::App;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let event_loop = EventLoop::new()?;
    let mut app = App::default();

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).map_err(|e| anyhow::anyhow!(e))?;

    Ok(())
}
