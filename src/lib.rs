#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(dead_code)] // TODO: @sashaweiss @nathanshelly remove this once MVP is working

extern crate clap;
extern crate crossbeam_channel;
extern crate rand;
extern crate tui;

mod driver;
mod food;
mod game;
mod snake;
mod terminal;

pub use driver::{Config, Driver};
