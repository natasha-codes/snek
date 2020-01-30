use crate::snek::game::{Game, GameDimensions};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::Rect;
use tui::widgets::{Block, Borders};
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
