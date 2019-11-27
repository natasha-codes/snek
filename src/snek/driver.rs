use crate::snek::game::Game;
use crate::snek::terminal::Terminal;

#[derive(Debug)]
pub struct Driver {
  term: Terminal,
  game: Game,
}

impl Driver {
  pub fn new() -> Self {
    Driver {
      term: Terminal::new(),
      game: Game::new(),
    }
  }

  pub fn drive(&mut self) {
    match self.term.render(&mut self.game) {
      Ok(_) => {}
      Err(err) => println!("{}", err),
    }
  }
}
