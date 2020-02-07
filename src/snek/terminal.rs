use crate::snek::food::Food;
use crate::snek::game::{Game, GameCoordinate, GameDimensions};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Terminal as TuiTerminal;

pub(crate) struct Terminal {
  terminal:
    TuiTerminal<TermionBackend<AlternateScreen<RawTerminal<std::io::Stdout>>>>,
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

impl GameDimensions {
  fn fits_in(&self, rect: &Rect) -> bool {
    self.width < rect.width && self.height < rect.height
  }
}

impl Terminal {
  pub fn new() -> Self {
    let raw_stdout = std::io::stdout().into_raw_mode().unwrap();
    let stdout = AlternateScreen::from(raw_stdout);
    let backend = TermionBackend::new(stdout);
    let terminal = TuiTerminal::new(backend).unwrap();
    Terminal { terminal }
  }

  pub fn render(&mut self, game: &Game) -> Result<(), ()> {
    self
      .terminal
      .draw(|mut f| {
        assert!(
          game.dimensions().fits_in(&f.size()),
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
              x: *x,
              y: *y,
            },
          );
        }
      })
      .unwrap();

    Ok(())
  }

  pub fn size(&self) -> GameDimensions {
    // TODO: actually handle this
    let Rect { width, height, .. } = self.terminal.size().unwrap();
    GameDimensions { width, height }
  }
}
