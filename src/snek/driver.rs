use crate::snek::terminal::Terminal;

#[derive(Debug, Copy, Clone)]
pub struct Driver {
  term: Terminal,
}

#[allow(dead_code)] // TODO: remove
impl Driver {
  pub fn new() -> Self {
    Driver {
      term: Terminal::new(),
    }
  }

  pub fn drive(self) -> bool {
    self.term.foo()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_drive() {
    assert!(Driver::new().drive());
  }
}
