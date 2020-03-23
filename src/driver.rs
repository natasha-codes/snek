use crossbeam_channel::{
  self as crossbeam, bounded, TryRecvError, TrySendError,
};
use std::{io, thread, time};
use termion::event::Key;
use termion::input::TermRead;

use crate::game::Game;
use crate::terminal::Terminal;

pub struct Driver {
  term: Terminal,
  game: Game,
  paused: bool,
}

pub type Result<T> = std::result::Result<T, String>;

pub(crate) enum UserAction {
  Move(Direction),
  Quit,
  PauseResume,
  None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Direction {
  North,
  South,
  East,
  West,
}

#[derive(Debug)]
pub struct GameConfig {
  pub food_count: usize,
}

#[derive(Debug)]
pub struct Config {
  pub game_config: GameConfig,
}

impl Driver {
  pub fn play() -> Result<()> {
    Self::play_with_config(Config::default())
  }

  pub fn play_with_config(config: Config) -> Result<()> {
    Self::new(config).drive()
  }

  pub fn new(config: Config) -> Self {
    let term = Terminal::new();
    let game_space = term.game_space();

    Driver {
      term,
      paused: false,
      game: Game::new(config.game_config, game_space),
    }
  }

  pub fn drive(&mut self) -> Result<()> {
    // Set up channels
    let (key_send, key_recv) = bounded(1);
    let (tick_send, tick_recv) = bounded(1);
    let key_recv_game = key_recv.clone();

    // Set up key listener and game loop timer
    listen_for_keys(key_send, key_recv);
    tick_with_ms_delay(100, tick_send);

    // Update game state
    self.render()?;

    for _ in tick_recv.iter() {
      match key_recv_game.try_recv() {
        Ok(key) => {
          let user_action = UserAction::from(key);

          if let UserAction::Quit = user_action {
            break;
          } else {
            self.respond_to_action(user_action)?;
          }
        }
        Err(TryRecvError::Empty) => self.respond_to_action(UserAction::None)?,
        Err(TryRecvError::Disconnected) => {
          return Err(String::from("Key channel disconnected"));
        }
      }
    }

    Ok(())
  }

  fn respond_to_action(&mut self, user_action: UserAction) -> Result<()> {
    if self.paused {
      if let UserAction::PauseResume = user_action {
        self.paused = false
      }
    } else {
      match user_action {
        UserAction::Quit => unreachable!("Quit action should be handled above"),
        UserAction::PauseResume => self.paused = true,
        UserAction::None | UserAction::Move(_) => {
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

/// `send` and `recv` must be attached to the same channel.
fn listen_for_keys(
  key_send: crossbeam::Sender<Key>,
  key_recv: crossbeam::Receiver<Key>,
) {
  let mut keys = io::stdin().keys();

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
}

fn tick_with_ms_delay(ms: u64, tick_send: crossbeam::Sender<()>) {
  thread::spawn(move || {
    while let Ok(_) = tick_send.send(()) {
      let delay = time::Duration::from_millis(ms);
      thread::sleep(delay);
    }
  });
}

impl Direction {
  pub fn inverted(self) -> Self {
    match self {
      Self::North => Self::South,
      Self::South => Self::North,
      Self::East => Self::West,
      Self::West => Self::East,
    }
  }
}

impl From<Key> for UserAction {
  fn from(key: Key) -> Self {
    match key {
      Key::Char('w') | Key::Up => Self::Move(Direction::North),
      Key::Char('s') | Key::Down => Self::Move(Direction::South),
      Key::Char('d') | Key::Right => Self::Move(Direction::East),
      Key::Char('a') | Key::Left => Self::Move(Direction::West),
      Key::Char(' ') => Self::PauseResume,
      Key::Esc | Key::Ctrl('c') => Self::Quit,
      _ => Self::None,
    }
  }
}

impl Default for GameConfig {
  fn default() -> Self {
    Self { food_count: 1 }
  }
}

impl Default for Config {
  fn default() -> Self {
    Self {
      game_config: GameConfig::default(),
    }
  }
}