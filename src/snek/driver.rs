use crate::snek::game::Game;
use crate::snek::terminal::Terminal;

#[derive(Debug, Clone)]
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

  pub fn drive(&self) {
    println!(
      "snek: {}, foods: {}",
      self.term.render(),
      self.game.count_food()
    );
  }
}
