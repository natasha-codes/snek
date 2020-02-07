use crate::snek::food::Food;
use crate::snek::game::{Game, GameCoordinate, GameDimensions};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Terminal as TuiTerminal;

#[derive(Debug, Clone, Copy)]
struct TerminalOffset {
  x: u16,
  y: u16,
}

pub(crate) struct Terminal {
  terminal:
    TuiTerminal<TermionBackend<AlternateScreen<RawTerminal<std::io::Stdout>>>>,
  game_space_offset: TerminalOffset,
}

impl Terminal {
  pub fn new() -> Self {
    let raw_stdout = std::io::stdout().into_raw_mode().unwrap();
    let stdout = AlternateScreen::from(raw_stdout);
    let backend = TermionBackend::new(stdout);
    let terminal = TuiTerminal::new(backend).unwrap();
    Terminal {
      terminal,
      game_space_offset: TerminalOffset { x: 1, y: 1 }, // adjust for borders
    }
  }

  pub fn render(&mut self, game: &Game) -> Result<(), ()> {
    let game_space_offset = self.game_space_offset;

    self
      .terminal
      .draw(|mut f| {
        eprintln!("{:?}, {:?}", game.dimensions(), f.size().border_adjusted());

        assert!(
          game.dimensions().fits_in(&f.size().border_adjusted()),
          "Terminal was larger than game - did the terminal screen resize?"
        );

        let mut block = Block::default().borders(Borders::ALL);
        f.render(&mut block, f.size());

        for (mut food, GameCoordinate { x, y }) in game.food() {
          f.render(
            &mut food,
            Rect {
              width: 1,
              height: 1,
              x: game_space_offset.x + *x,
              y: game_space_offset.y + *y,
            },
          );
        }
      })
      .unwrap();

    Ok(())
  }

  pub fn game_space(&self) -> GameDimensions {
    self.terminal.size().map(|r| r.border_adjusted()).unwrap()
  }
}

impl Food {
  fn repr(&self) -> char {
    match self {
      Food::Cake => 'c',
      Food::Cherry => 'y',
      Food::Mouse => 'm',
    }
  }
}

impl Widget for Food {
  fn draw(&mut self, area: Rect, buf: &mut Buffer) {
    let sprite = [Text::raw(self.repr().to_string())];

    Paragraph::new(sprite.iter()).draw(area, buf);
  }
}

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
