mod input;
mod snek;

use clap::Clap;

use crate::snek::Driver;
use input::Input;

fn main() {
  Driver::play_with_config(Input::parse().into())
    .map_err(|err| eprintln!("Driver returned with an error: {:?}", err))
    .unwrap();
}
