use crate::snek::game::{Game, GameCoordinate, GameDimensions};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::Rect;
use tui::widgets::{Block, Borders, Paragraph, Text};
use tui::Terminal as TuiTerminal;

pub(crate) struct Terminal {
  terminal:
    TuiTerminal<TermionBackend<AlternateScreen<RawTerminal<std::io::Stdout>>>>,
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
        let mut block = Block::default().borders(Borders::ALL);
        f.render(&mut block, f.size());

        let sprite = [Text::raw("a")];

        for (_, GameCoordinate { x, y }) in game.food() {
          f.render(
            &mut Paragraph::new(sprite.iter()),
            Rect {
              width: 1,
              height: 1,
              // TODO: there should be a cleaner way to do this
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
