mod app;

use app::App;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    let first_arg = std::env::args().nth(1).expect("no pattern given");

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App::new(first_arg);

    loop {
        if app.remain() < 0 {
            break;
        }
        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(48),
                    Constraint::Max(1),
                    Constraint::Max(1),
                ])
                .split(frame.size());

            let text = format!("{} - {} {}", app.remain_seconds(), '󰂚', app.bell_time());
            frame.render_widget(
                Paragraph::new(text).block(
                    Block::default()
                        .borders(Borders::NONE)
                        .padding(Padding::new(4, 4, 0, 0)),
                ),
                layout[1],
            );

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
