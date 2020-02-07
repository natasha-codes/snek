use crate::snek::game::Game;
use crate::snek::terminal::Terminal;
use std::io;
use termion::event::Key;
use termion::input::TermRead;

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
    let g = Game::new(self.term.game_space());

    match &mut self.term.render(&g) {
      Ok(_) => {}
      _ => println!("fuck"),
    }

    let mut keys = io::stdin().keys();
    while let Some(Ok(key)) = keys.next() {
      match key {
        Key::Esc | Key::Char('q') | Key::Ctrl('c') => break,
        _ => {}
      }
    }
  }
}
