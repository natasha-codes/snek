pub(crate) trait TerminalRenderable {
  fn as_string(&self) -> String;
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Terminal {}

impl Terminal {
  pub fn new() -> Self {
    Terminal {}
  }

  pub fn render<Renderable: TerminalRenderable>(
    &self,
    renderable: &Renderable,
  ) -> bool {
    println!("{}", renderable.as_string());
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestRenderable {}
  impl TerminalRenderable for TestRenderable {
    fn as_string(&self) -> String {
      String::from("this is a test")
    }
  }

  #[test]
  fn test_render() {
    assert_eq!(Terminal::new().render(&TestRenderable {}), true);
  }
}
