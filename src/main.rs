// #![cfg_attr(debug_assertions, allow(unused))]
#![allow(unsafe_op_in_unsafe_fn)]
#![feature(associated_type_defaults)]

use anyhow::Result;

mod app;

fn main() -> Result<()> {
    pretty_env_logger::init();
    app::AppWrapper::run_default()?;

    Ok(())
}
