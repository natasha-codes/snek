use crate::snek::game::Game;
use crate::snek::terminal::Terminal;

pub struct Driver {
  term: Terminal,
}

impl Driver {
  pub fn new() -> Self {
    Driver {
      term: Terminal::new(),
    }
  }

  pub fn drive(&mut self) {
    let g = Game::new();

    match &mut self.term.render(&g) {
      Ok(_) => {}
      _ => println!("fuck"),
    }
  }
}
