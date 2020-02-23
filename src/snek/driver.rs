use crate::snek::game::Game;
use crate::snek::terminal::Terminal;
use crossbeam_channel::{bounded, TryRecvError, TrySendError};
use std::{io, thread, time};
use termion::event::Key;
use termion::input::TermRead;

pub struct Driver {
  term: Terminal,
  game: Game,
  paused: bool,
}

pub type Result<T> = std::result::Result<T, String>;

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
    let term = Terminal::new();
    let game_space = term.game_space();

    Driver {
      term,
      paused: false,
      game: Game::new(game_space),
    }
  }

  pub fn drive(&mut self) -> Result<()> {
    let mut keys = io::stdin().keys();

    self.render()?;

    // Listen for input

    let (key_send, key_recv) = bounded(0);
    let key_recv_game = key_recv.clone();

    thread::spawn(move || {
      while let Some(Ok(key)) = keys.next() {
        // Remove previously-sent key, if still in the channel
        match key_recv.try_recv() {
          Ok(_) | Err(TryRecvError::Empty) => {}
          Err(TryRecvError::Disconnected) => break,
        }

        // Send newly received key
        match key_send.try_send(key) {
          Ok(_) => {}
          Err(TrySendError::Full(_)) => {
            unreachable!("Should-be-empty channel was full")
          }
          Err(TrySendError::Disconnected(_)) => break,
        }
      }
    });

    // Manage the game-loop timer

    let (tick_send, tick_recv) = bounded(0);

    thread::spawn(move || {
      while let Ok(_) = tick_send.send(()) {
        let delay = time::Duration::from_millis(300);
        thread::sleep(delay);
      }
    });

    // Update game state

    for _ in tick_recv.iter() {
      match key_recv_game.try_recv() {
        Ok(key) => self.respond_to_key(Some(key))?,
        Err(TryRecvError::Empty) => self.respond_to_key(None)?,
        Err(TryRecvError::Disconnected) => {
          return Err(String::from("Key channel disconnected"));
        }
      }
    }

    Ok(())
  }

  fn respond_to_key(&mut self, key: Option<Key>) -> Result<()> {
    let user_action = UserAction::from(key);

    if self.paused {
      if let UserAction::PauseResume = user_action {
        self.paused = false
      }
    } else {
      match user_action {
        UserAction::Quit => {}
        UserAction::PauseResume => self.paused = true,
        UserAction::None
        | UserAction::MoveNorth
        | UserAction::MoveSouth
        | UserAction::MoveEast
        | UserAction::MoveWest => {
          self.game.update_for_user_action(user_action);
          self.render()?;
        }
      }
    }

    Ok(())
  }

  fn render(&mut self) -> Result<()> {
    self
      .term
      .render(&self.game)
      .map_err(|err| format!("Failed to render game: {:?}", err))
  }
}

impl From<Option<Key>> for UserAction {
  fn from(maybe_key: Option<Key>) -> Self {
    match maybe_key {
      Some(key) => Self::from(key),
      None => Self::None,
    }
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
