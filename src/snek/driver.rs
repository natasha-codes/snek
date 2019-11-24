#[derive(Debug, Copy, Clone)]
pub struct Driver {}

#[allow(dead_code)] // TODO: remove
impl Driver {
  pub fn new() -> Self {
    Driver {}
  }

  fn foo(self) -> bool {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn drive() {
    assert!(Driver::new().foo());
  }
}
