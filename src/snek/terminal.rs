use std::io::Write;

pub(crate) trait TerminalRenderable {
  fn as_string(&self) -> String;
}

#[derive(Debug)]
pub(crate) struct Terminal {
  out_stream: std::io::Stdout,
}

impl Terminal {
  pub fn new() -> Self {
    Terminal {
      out_stream: std::io::stdout(),
    }
  }

  pub fn render<Renderable: TerminalRenderable>(
    &mut self,
    renderable: &Renderable,
  ) -> Result<(), std::io::Error> {
    writeln!(&mut self.out_stream, "{}", renderable.as_string())
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
    assert!(Terminal::new().render(&TestRenderable {}).is_ok());
  }
}
