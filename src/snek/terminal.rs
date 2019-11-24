#[derive(Debug, Copy, Clone)]
pub(crate) struct Terminal {}

impl Terminal {
  pub fn new() -> Self {
    Terminal {}
  }

  pub fn render(&self) -> &str {
    "🐍"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_render() {
    assert_eq!(Terminal::new().render(), "🐍");
  }
}
