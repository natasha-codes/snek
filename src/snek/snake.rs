use crate::snek::driver::Direction;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub(crate) struct Snake {
  // The direction of a body segment is "how to get to the previous body segment/head from me".
  // E.g., if the first segment is `North`, this implies the head is north of the first segment.
  segments: VecDeque<Direction>,
}

impl Snake {
  pub fn new() -> Self {
    Snake {
      segments: VecDeque::new(),
    }
  }

  pub fn iter(&self) -> impl Iterator<Item = Direction> + '_ {
    self.segments.iter().copied()
  }

  pub fn advance(&mut self, direction: Direction) {
    self.grow(direction);
    self.drop_tail();
  }

  pub fn grow(&mut self, direction: Direction) {
    self.segments.push_front(direction);
  }

  fn drop_tail(&mut self) {
    self.segments.pop_back();
  }
}

#[cfg(test)]
mod tests {
  use super::Direction::*;
  use super::*;

  impl Snake {
    fn with_directions(directions: &[Direction]) -> Snake {
      let mut snek = Snake::new();

      for direction in directions {
        snek.grow(*direction);
      }

      snek
    }

    fn length(&self) -> usize {
      self.segments.len() + 1
    }
  }

  #[test]
  fn test_new_snake() {
    assert_eq!(Snake::new().length(), 1);
  }

  #[test]
  fn test_grow_snake() {
    let mut snek = Snake::with_directions(&[North, North, North]);
    assert_eq!(snek.length(), 4);

    snek.grow(East);
    assert_eq!(snek.length(), 5);
    assert_eq!(snek.segments.front(), Some(&East));
  }

  #[test]
  fn test_advance_snake() {
    let mut snek = Snake::with_directions(&[North, North, North]);
    assert_eq!(snek.length(), 4);

    snek.advance(East);
    assert_eq!(snek.length(), 4);
    assert_eq!(snek.segments.front(), Some(&East));
  }

  #[test]
  fn test_iter_snake() {
    let directions = vec![North, East, North];
    let snek = Snake::with_directions(&directions);

    assert_eq!(directions, snek.iter().collect::<Vec<Direction>>());
  }
}
