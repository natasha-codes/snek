use crate::snek::driver::{Direction, UserAction};
use crate::snek::food::Food;
use crate::snek::snake::Snake;
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
  direction: Direction,
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
    snake.grow(Direction::North);
    snake.grow(Direction::North);
    snake.grow(Direction::North);
    snake.grow(Direction::East);
    snake.grow(Direction::East);
    snake.grow(Direction::East);

    Game {
      dimensions,
      snek: snake,
      snek_head: SnakeHead {
        location: dimensions.center(),
        direction: Direction::North,
      },
      food,
    }
  }

  // MARK: - Getters

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
    impl Iterator<Item = (Direction, GameCoordinate)> + '_,
  ) {
    let mut curr = self.snek_head.location;

    (
      self.snek_head.location,
      self.snek.iter().map(move |dir| {
        // Remember that (0, 0) is at the top-left corner
        match dir {
          Direction::North => curr.y += 1,
          Direction::South => curr.y -= 1,
          Direction::East => curr.x -= 1,
          Direction::West => curr.x += 1,
        }

        (dir, curr)
      }),
    )
  }

  // MARK: - Actions

  pub fn update_for_user_action(&mut self, user_action: UserAction) {
    match user_action {
      UserAction::None => self.advance_snake(),
      UserAction::Move(move_direction) => {
        if move_direction == self.snek_head.direction.inverted() {
          self.advance_snake();
        } else {
          self.advance_snake_in_direction(move_direction)
        }
      }
      UserAction::Quit | UserAction::PauseResume => {}
    };
  }

  fn advance_snake(&mut self) {
    self.advance_snake_in_direction(self.snek_head.direction);
  }

  fn advance_snake_in_direction(&mut self, direction: Direction) {
    self.snek.advance(direction);
    self.snek_head.update_for_move_in_direction(direction);
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
}

impl SnakeHead {
  fn update_for_move_in_direction(&mut self, direction: Direction) {
    self.direction = direction;

    // Remember that (0, 0) is at the top-left corner
    match direction {
      Direction::North => self.location.y -= 1,
      Direction::South => self.location.y += 1,
      Direction::East => self.location.x += 1,
      Direction::West => self.location.x -= 1,
    }
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
