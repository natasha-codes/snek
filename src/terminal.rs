use std::io;
use termion::cursor::HideCursor;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::terminal::{Frame, Terminal as TuiTerminal};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};

use crate::driver::Direction;
use crate::food::Food;
use crate::game::{Game, GameCoordinate, GameDimensions};

#[derive(Debug, Clone, Copy)]
struct TerminalOffset {
  x: u16,
  y: u16,
}

type OurBackend =
  TermionBackend<HideCursor<AlternateScreen<RawTerminal<std::io::Stdout>>>>;

pub(crate) struct Terminal {
  terminal: TuiTerminal<OurBackend>,
  game_space_offset: TerminalOffset,
}

impl Terminal {
  pub fn new() -> Self {
    let stdout = std::io::stdout().into_raw_mode().unwrap();
    let stdout = AlternateScreen::from(stdout);
    let stdout = HideCursor::from(stdout);
    let backend = TermionBackend::new(stdout);
    let terminal = TuiTerminal::new(backend).unwrap();
    Terminal {
      terminal,
      game_space_offset: TerminalOffset { x: 1, y: 1 }, // adjust for borders
    }
  }

  pub fn render(&mut self, game: &Game) -> io::Result<()> {
    let game_space_offset = self.game_space_offset;
    let adjust = |GameCoordinate { x, y }: GameCoordinate| -> GameCoordinate {
      GameCoordinate {
        x: x + game_space_offset.x,
        y: y + game_space_offset.y,
      }
    };

    self.terminal.draw(|mut f| {
      assert!(
        game.dimensions().fits_in(&f.size().border_adjusted()),
        "Terminal was larger than game - did the terminal screen resize?"
      );

      let mut block = Block::default().borders(Borders::ALL);
      f.render(&mut block, f.size());

      for (food, coordinate) in game.food() {
        f.render_char_widget(food, adjust(coordinate));
      }

      let (head, body) = game.snake_bits();

      f.render_char_widget(head, adjust(head));
      for (direction, coordinate) in body {
        f.render_char_widget(direction, adjust(coordinate));
      }
    })?;

    Ok(())
  }

  pub fn game_space(&self) -> GameDimensions {
    self.terminal.size().map(|r| r.border_adjusted()).unwrap()
  }
}

// MARK: - Sprite rendering

struct CharWidget(char);

impl Widget for CharWidget {
  fn draw(&mut self, area: Rect, buf: &mut Buffer) {
    let sprite = [Text::raw(self.0.to_string())];

    Paragraph::new(sprite.iter()).draw(area, buf);
  }
}

impl From<char> for CharWidget {
  fn from(c: char) -> Self {
    Self(c)
  }
}

impl Into<CharWidget> for Food {
  fn into(self) -> CharWidget {
    match self {
      Food::Cake => 'c',
      Food::Cherry => 'y',
      Food::Mouse => 'm',
    }
    .into()
  }
}

impl Into<CharWidget> for Direction {
  fn into(self) -> CharWidget {
    match self {
      Direction::North => '^',
      Direction::South => 'v',
      Direction::East => '>',
      Direction::West => '<',
    }
    .into()
  }
}

// The snake head is represented by a `GameCoordinate` - this impl refers to that usage.
impl Into<CharWidget> for GameCoordinate {
  fn into(self) -> CharWidget {
    '⦾'.into()
  }
}

trait CharWidgetRenderer<I: Into<CharWidget>> {
  fn render_char_widget(&mut self, widget: I, at: GameCoordinate);
}

impl<'a, I: Into<CharWidget>> CharWidgetRenderer<I> for Frame<'a, OurBackend> {
  fn render_char_widget(
    &mut self,
    widget: I,
    GameCoordinate { x, y }: GameCoordinate,
  ) {
    self.render::<CharWidget>(
      &mut widget.into(),
      Rect {
        width: 1,
        height: 1,
        x,
        y,
      },
    )
  }
}

// MARK: - Mapping terminal -> game space

trait BorderAdjustable {
  fn border_adjusted(&self) -> GameDimensions;
}

impl BorderAdjustable for Rect {
  fn border_adjusted(&self) -> GameDimensions {
    GameDimensions {
      width: self.width - 2,
      height: self.height - 2,
    }
  }
}

impl GameDimensions {
  fn fits_in(&self, rhs: &Self) -> bool {
    self.width <= rhs.width && self.height <= rhs.height
  }
}
