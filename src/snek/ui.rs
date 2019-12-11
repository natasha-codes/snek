use crate::snek::game::Game;
use crate::snek::terminal::TerminalRenderable;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct UI {
  food_count: usize,
  snake_length: usize,
}

impl UI {
  fn new(game: &Game) -> Self {
    UI {
      food_count: game.count_food(),
      snake_length: game.snake_length(),
    }
  }
}

impl From<&Game> for UI {
  fn from(game: &Game) -> Self {
    Self::new(game)
  }
}

impl TerminalRenderable for UI {
  fn as_string(&self) -> String {
    format!(
      "Food count: {}, snake length: {}",
      self.food_count, self.snake_length
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_from() {
    assert_eq!(UI::from(&Game::new()), UI::new(&Game::new()))
  }

  #[test]
  fn test_renderable() {
    assert_eq!(
      UI::new(&Game::new()).as_string(),
      "Food count: 3, snake length: 1"
    );
  }
}
