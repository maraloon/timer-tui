mod arg_resolver;
mod fmt;
mod timer;

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
use timer::Timer;

fn main() -> Result<()> {
    let first_arg = std::env::args().nth(1).expect("no pattern given");

    let duration_ms = match arg_resolver::parse_time_argument(first_arg) {
        Ok(d) => d,
        Err(err) => {
            eprint!("Error: {}", err);
            std::process::exit(1);
        }
    };

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut timer = Timer::new(duration_ms);

    loop {
        timer.tick();
        if timer.remain_ms < 0 {
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

            let text = format!(
                "{} - 󰂚 {}",
                fmt::remain_time_string(&mut timer),
                fmt::finish_time_string(&mut timer)
            );

            let text_color = match timer.paused {
                true => Color::Rgb(44, 56, 54),
                false => Color::Rgb(255, 0, 0),
            };
            let progress_color = match timer.paused {
                true => Color::Rgb(44, 56, 54),
                false => Color::Rgb(0, 208, 152)
            };

            frame.render_widget(
                Paragraph::new(text)
                    .block(
                        Block::default()
                            .borders(Borders::NONE)
                            .padding(Padding::new(1, 1, 0, 0)),
                    )
                    .style(Style::default().fg(text_color)),
                layout[1],
            );

            frame.render_widget(
                Gauge::default()
                    .label("")
                    .use_unicode(true)
                    .block(
                        Block::default()
                            .borders(Borders::NONE)
                            .padding(Padding::new(1, 1, 0, 0)),
                    )
                    .gauge_style(
                        Style::default()
                            .fg(progress_color)
                            .bg(Color::Black),
                    )
                    .ratio(fmt::ratio(&mut timer)),
                layout[2],
            );
        })?;

        if event::poll(std::time::Duration::from_millis(10))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('p') => timer.toggle_pause(),
                        KeyCode::Char('q') => break,
                        _ => todo!(),
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
