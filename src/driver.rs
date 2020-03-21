use crossbeam_channel::{self as crossbeam};
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

#[derive(Debug)]
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

  fn drive(&mut self) -> Result<()> {
    // Set up key listener and game loop timer
    let (_, key_recv) = Driver::listen_for_keys();
    let (_, tick_recv) = Driver::tick_with_ms_delay(100);

    // Render initial game state
    self.render()?;

    // Check for a keypress, once per tick
    for _ in tick_recv.iter() {
      match key_recv.try_recv() {
        // A key was pressed
        Ok(key) => {
          let user_action = UserAction::from(key);

          if let UserAction::Quit = user_action {
            break;
          } else {
            self.respond_to_action(user_action)?;
          }
        }
        // No key was pressed
        Err(crossbeam::TryRecvError::Empty) => {
          self.respond_to_action(UserAction::None)?
        }
        // The keypress channel disconnected - this is bad
        Err(crossbeam::TryRecvError::Disconnected) => {
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

  /// Spawn a thread to listen for keypresses on `stdin`.
  ///
  /// The returned receiver will only ever hold at most one keypress. If a
  /// keypress occurs while a previously-pressed key is still waiting in
  /// the channel, we will remove it and replace it with the new keypress.
  fn listen_for_keys() -> (thread::JoinHandle<()>, crossbeam::Receiver<Key>) {
    let (key_send, key_recv) = crossbeam::bounded(1);
    let key_recv_ret = key_recv.clone();

    let join_handle = thread::spawn(move || {
      let mut keys = io::stdin().keys();

      while let Some(Ok(key)) = keys.next() {
        // Remove previously-sent key, if still in the channel
        match key_recv.try_recv() {
          Ok(_) | Err(crossbeam::TryRecvError::Empty) => {}
          Err(crossbeam::TryRecvError::Disconnected) => break,
        }

        // Send newly received key
        match key_send.try_send(key) {
          Ok(_) => {}
          Err(crossbeam::TrySendError::Full(_)) => {
            unreachable!("Should-be-empty channel was full")
          }
          Err(crossbeam::TrySendError::Disconnected(_)) => break,
        }
      }
    });

    (join_handle, key_recv_ret)
  }

  /// Spawn a thread to send a tick every `ms` milliseconds.
  fn tick_with_ms_delay(
    ms: u64,
  ) -> (thread::JoinHandle<()>, crossbeam::Receiver<()>) {
    let (tick_send, tick_recv) = crossbeam::bounded(1);

    let join_handle = thread::spawn(move || {
      while let Ok(_) = tick_send.send(()) {
        let delay = time::Duration::from_millis(ms);
        thread::sleep(delay);
      }
    });

    (join_handle, tick_recv)
  }
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
