mod app;

use app::App;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{canvas::Label, *},
};
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
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(48), Constraint::Max(1), Constraint::Max(1)])
                .split(frame.size());

            let from = app.from.to_string();
            let to = app.to.to_string();
            let remain = app.remainSeconds();
            let passed = app.passed();

            let text = format!(
                "remain {}, passed {}, ratio {}",
                remain+1, passed, app.ratio()
            );
            frame.render_widget(Paragraph::new(text), layout[1]);

            frame.render_widget(
                Gauge::default()
                    .label("")
                    .use_unicode(true)
                    .block(
                        Block::default()
                            .borders(Borders::NONE)
                            .padding(Padding::new(4, 4, 0, 0)),
                    )
                    .gauge_style(Style::default().fg(Color::Yellow).bg(Color::Black))
                    .ratio(app.ratio()),
                layout[2],
            );
        })?;

        if event::poll(std::time::Duration::from_millis(10))? {
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
