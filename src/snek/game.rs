use crate::snek::food::Food;
use crate::snek::snake::Snake;
use crate::snek::terminal::TerminalRenderable;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub(crate) struct Game {
  snek: Snake,
  food: Vec<Food>,
}

impl Game {
  pub fn new() -> Self {
    Game {
      snek: Snake::new(),
      food: vec![Food::Cake, Food::Cherry, Food::Mouse],
    }
  }

  pub fn count_food(&self) -> usize {
    self.food.len()
  }
}

impl TerminalRenderable for Game {
  fn as_string(&self) -> String {
    String::from_str("🐍").expect("Failed to parse snek emoji")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_game() {
    assert_eq!(Game::new().count_food(), 3);
  }
}
