mod app;

use app::App;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::{fmt::format, io::{stdout, Result}};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let app = App::new();

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let from = app.from.elapsed().as_secs().to_string();
            let to = app.from.elapsed().as_secs().to_string();
            let text = format!("From: {}, to {}", from, to);
            frame.render_widget(
                Paragraph::new(text)
                    .white()
                    .on_black(),
                area,
            );
        })?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
