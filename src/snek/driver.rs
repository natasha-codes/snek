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

  pub fn drive(self) {
    println!("{}", self.term.render());
  }
}
