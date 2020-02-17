use crate::snek::game::Game;
use crate::snek::terminal::Terminal;
use std::io;
use termion::event::Key;
use termion::input::TermRead;

pub struct Driver {
  term: Terminal,
}

pub(crate) enum UserAction {
  MoveNorth,
  MoveSouth,
  MoveEast,
  MoveWest,
  Quit,
  PauseResume,
  None,
}

impl Driver {
  pub fn new() -> Self {
    Driver {
      term: Terminal::new(),
    }
  }

  pub fn drive(&mut self) -> Result<(), ()> {
    let mut keys = io::stdin().keys();
    let mut game = Game::new(self.term.game_space());
    let mut paused = false;

    self.render(&game)?;

    while let Some(Ok(key)) = keys.next() {
      let user_action = UserAction::from(key);

      if paused {
        if let UserAction::PauseResume = user_action {
          paused = false
        }
      } else {
        match user_action {
          UserAction::None => {}
          UserAction::Quit => break,
          UserAction::PauseResume => paused = true,
          UserAction::MoveNorth
          | UserAction::MoveSouth
          | UserAction::MoveEast
          | UserAction::MoveWest => {
            game.update_for_user_action(user_action);
            self.render(&game)?;
          }
        }
      }
    }

    Ok(())
  }

  fn render(&mut self, game: &Game) -> Result<(), ()> {
    self
      .term
      .render(game)
      .map_err(|err| eprintln!("Failed to render game: {:?}", err))
  }
}

impl From<Key> for UserAction {
  fn from(key: Key) -> Self {
    match key {
      Key::Char('w') | Key::Up => Self::MoveNorth,
      Key::Char('s') | Key::Down => Self::MoveSouth,
      Key::Char('d') | Key::Right => Self::MoveEast,
      Key::Char('a') | Key::Left => Self::MoveWest,
      Key::Char(' ') => Self::PauseResume,
      Key::Esc | Key::Ctrl('c') => Self::Quit,
      _ => Self::None,
    }
  }
}
