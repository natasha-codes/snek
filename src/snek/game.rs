use crate::snek::food::Food;
use crate::snek::snake::Snake;

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

  pub fn snake_length(&self) -> usize {
    self.snek.length()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_game() {
    let game = Game::new();

    assert_eq!(game.count_food(), 3);
    assert_eq!(game.snake_length(), 1);
  }
}
