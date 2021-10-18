#![feature(
    format_args_capture,
    async_closure,
    map_first_last,
    box_patterns,
    concat_idents
)]
#![feature(error_iter)]
#![allow(type_alias_bounds)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate anyhow;

#[macro_use]
pub mod error;

pub mod cli;

pub mod datasource;
pub mod sampler;
pub mod utils;
pub mod version;
