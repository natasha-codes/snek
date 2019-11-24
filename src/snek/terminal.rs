#[derive(Debug, Copy, Clone)]
pub(crate) struct Terminal {}

#[allow(dead_code)] // TODO: remove
impl Terminal {
  pub fn new() -> Self {
    Terminal {}
  }

  pub fn foo(self) -> bool {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn drive() {
    assert!(Terminal::new().foo());
  }
}
