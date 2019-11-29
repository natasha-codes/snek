#[derive(Debug, Copy, Clone)]
pub(crate) struct Snake {
  length: usize,
}

impl Snake {
  pub fn new() -> Self {
    Snake { length: 1 }
  }

  pub fn length(&self) -> usize {
    self.length
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_snake() {
    assert_eq!(Snake::new().length(), 1);
  }
}
