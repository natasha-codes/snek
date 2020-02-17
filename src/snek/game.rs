use crate::snek::food::Food;
use crate::snek::snake::{Snake, SnakeDirection};
use rand::Rng;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct GameDimensions {
  pub width: u16,
  pub height: u16,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct GameCoordinate {
  pub x: u16,
  pub y: u16,
}

#[derive(Debug, Copy, Clone)]
struct SnakeHead {
  location: GameCoordinate,
  direction: SnakeDirection,
}

#[derive(Debug, Clone)]
pub(crate) struct Game {
  dimensions: GameDimensions,
  snek: Snake,
  snek_head: SnakeHead,
  food: Vec<(Food, GameCoordinate)>,
}

impl Game {
  pub fn new(dimensions: GameDimensions) -> Self {
    let food = vec![Food::Cake, Food::Cherry, Food::Mouse]
      .into_iter()
      .map(|food| (food, GameCoordinate::random(dimensions)))
      .collect();

    let mut snake = Snake::new();
    snake.grow(SnakeDirection::North);
    snake.grow(SnakeDirection::North);
    snake.grow(SnakeDirection::North);
    snake.grow(SnakeDirection::East);
    snake.grow(SnakeDirection::East);
    snake.grow(SnakeDirection::East);

    Game {
      dimensions,
      snek: snake,
      snek_head: SnakeHead {
        location: dimensions.center(),
        direction: SnakeDirection::North,
      },
      food,
    }
  }

  pub fn dimensions(&self) -> GameDimensions {
    self.dimensions
  }

  pub fn food(&self) -> impl Iterator<Item = (Food, GameCoordinate)> + '_ {
    self.food.iter().copied()
  }

  pub fn snake_bits(
    &self,
  ) -> (
    GameCoordinate,
    impl Iterator<Item = (SnakeDirection, GameCoordinate)> + '_,
  ) {
    let mut curr = self.snek_head.location;

    (
      self.snek_head.location,
      self
        .snek
        .iter()
        .map(move |dir| (dir, curr.update_with_direction(dir))),
    )
  }
}

impl GameDimensions {
  fn center(&self) -> GameCoordinate {
    GameCoordinate {
      x: self.width / 2,
      y: self.height / 2,
    }
  }
}

impl GameCoordinate {
  fn random(GameDimensions { width, height }: GameDimensions) -> Self {
    let mut rng = rand::thread_rng();
    GameCoordinate {
      x: rng.gen_range(0, width),
      y: rng.gen_range(0, height),
    }
  }

  fn update_with_direction(&mut self, direction: SnakeDirection) -> Self {
    use SnakeDirection::*;
    match direction {
      North => self.y += 1,
      South => self.y -= 1,
      East => self.x += 1,
      West => self.x -= 1,
    };

    *self
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

    assert_eq!(game.food().count(), 3);
    assert_eq!(game.dimensions(), GameDimensions { width, height });
  }
}
