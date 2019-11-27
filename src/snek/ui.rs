use crate::snek::game::Game;
use crate::snek::terminal::TerminalRenderable;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct UI {
  food_count: usize,
}

impl UI {
  fn new(game: &Game) -> Self {
    UI {
      food_count: game.count_food(),
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
    format!("Food: {}", self.food_count)
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
    assert_eq!(UI::new(&Game::new()).as_string(), "Food: 3");
  }
}
