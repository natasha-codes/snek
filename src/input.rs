use clap::Clap;

use crate::snek::GameConfig;

/// Play the classic game snake! 🐍
#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "Nathan Shelly & Sasha Weiss")]
pub struct Input {
  /// Sets the amount of food on the board at any given time
  #[clap(long = "food-count")]
  food_count: Option<usize>,
}

impl Into<GameConfig> for Input {
  fn into(self) -> GameConfig {
    let mut builder = GameConfig::default();

    if let Some(food_count) = self.food_count {
      builder.food_count = food_count
    }

    builder
  }
}
