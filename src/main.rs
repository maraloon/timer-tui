mod app;

use app::App;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::{
    fmt::format,
    io::{stdout, Result},
};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App::new();

    loop {
        terminal.draw(|frame| {
            // let area = frame.size();
            let from = app.from.to_string();
            let to = app.to.to_string();
            let remain = app.remain();
            let passed = app.passed();

            let area = Rect::new(0, 0, frame.size().width, 1);
            let text = format!(
                "From: {}, to {}, remain {}, passed {}",
                from, to, remain, passed
            );
            frame.render_widget(Paragraph::new(text), area);

            let area = Rect::new(0, 1, frame.size().width, 1);

            frame.render_widget(
                Gauge::default()
                    .block(Block::default().borders(Borders::NONE))
                    .gauge_style(
                        Style::default()
                            .fg(Color::White)
                            .bg(Color::Black)
                            .add_modifier(Modifier::ITALIC),
                    )
                    .ratio(app.ratio()),
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
