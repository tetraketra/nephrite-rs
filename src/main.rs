// #![cfg_attr(debug_assertions, allow(unused))]
#![allow(unsafe_op_in_unsafe_fn)]

use anyhow::Result;
use winit::event_loop::{ControlFlow, EventLoop};

mod app;
use app::App;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let event_loop = EventLoop::new()?;
    let mut app_wrapper = app::AppWrapper::default();
    event_loop.run_app(&mut app_wrapper)?;

    Ok(())
}
