use crate::snek::food::Food;
use crate::snek::snake::Snake;
use rand::Rng;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct GameDimensions {
  pub width: u16,
  pub height: u16,
}

#[derive(Debug, Clone)]
pub(crate) struct GameCoordinate {
  pub x: u16,
  pub y: u16,
}

impl GameCoordinate {
  pub fn random(GameDimensions { width, height }: GameDimensions) -> Self {
    let mut rng = rand::thread_rng();
    GameCoordinate {
      x: rng.gen_range(0, width),
      y: rng.gen_range(0, height),
    }
  }
}

#[derive(Debug, Clone)]
pub(crate) struct Game {
  dimensions: GameDimensions,
  snek: Snake,
  food: Vec<(Food, GameCoordinate)>,
}

impl Game {
  pub fn new(dimensions: GameDimensions) -> Self {
    let food = vec![Food::Cake, Food::Cherry, Food::Mouse]
      .into_iter()
      .map(|food| (food, GameCoordinate::random(dimensions)))
      .collect();
    Game {
      dimensions,
      snek: Snake::new(),
      food,
    }
  }

  pub fn food(&self) -> &Vec<(Food, GameCoordinate)> {
    &self.food
  }

  pub fn snake_length(&self) -> usize {
    self.snek.length()
  }

  fn dimensions(&self) -> GameDimensions {
    self.dimensions
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_game() {
    let width = 10;
    let height = 10;

    let game = Game::new(GameDimensions { width, height });

    assert_eq!(game.food().len(), 3);
    assert_eq!(game.snake_length(), 1);
    assert_eq!(game.dimensions(), GameDimensions { width, height });
  }
}
