#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(dead_code)] // TODO: @sashaweiss @nathanshelly remove this once MVP is working

extern crate rand;
extern crate tui;

pub mod driver;
mod food;
mod game;
mod snake;
mod terminal;
