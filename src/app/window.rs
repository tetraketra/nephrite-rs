use std::marker::PhantomData;

use anyhow::{Context as _Ctx, Result};
use image::GenericImageView;
use winit::{
    event_loop::ActiveEventLoop,
    window::{Icon, Window as WinitWindow},
};

use crate::app::{Initialized, State, Uninitialized};

pub struct WindowData {
    pub window: WinitWindow,
}

pub struct Window<S, D = ()>
where
    S: State,
{
    pub data: D,
    _marker:  PhantomData<S>,
}

impl<S: State, D: Default> Default for Window<S, D> {
    fn default() -> Self {
        Self {
            data:    D::default(),
            _marker: PhantomData,
        }
    }
}

impl Window<Uninitialized, ()> {
    pub fn initialize(event_loop: &ActiveEventLoop) -> Result<Window<Initialized, WindowData>> {
        let icon_bytes = include_bytes!("../../assets/icon_jade_1.png");
        let icon_mem = image::load_from_memory(icon_bytes)
            .inspect_err(|e| event_loop.exit())
            .with_context(|| "Failed to load app icon asset")?;
        let (width, height) = icon_mem.dimensions();
        let icon_rgba = icon_mem.to_rgba8().into_raw();
        let icon = Icon::from_rgba(icon_rgba, width, height)
            .inspect_err(|e| event_loop.exit())
            .with_context(|| "Failed to turn app icon asset into icon")?;

        let window = event_loop
            .create_window(
                WinitWindow::default_attributes()
                    .with_title("Nephrite")
                    .with_window_icon(Some(icon)),
            )
            .inspect_err(|e| event_loop.exit())
            .with_context(|| "Failed to create window")?;

        Ok(Window {
            data: WindowData { window },

            _marker: PhantomData,
        })
    }
}
