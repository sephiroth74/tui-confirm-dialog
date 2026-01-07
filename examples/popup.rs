use std::{error::Error, io};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::text::Line;
use ratatui::{prelude::*, widgets::*};

#[derive(Default)]
struct App {
    popup_opened: bool,
}

impl App {
    fn new() -> App {
        App::default()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // create app and run it
    let app = App::new();
    let res = run_app(app);

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app(mut app: App) -> io::Result<()> {
    ratatui::run(|terminal| loop {
        terminal.draw(|f| ui(f, &mut app)).expect("panic message");;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Char('p') => app.popup_opened = !app.popup_opened,
                    _ => {}
                }
            }
        }
    })
}

fn ui(f: &mut Frame, app: &mut App) {
    let area = f.area();

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(area);

    let text = if app.popup_opened {
        "Press `p` to close the popup"
    } else {
        "Press `p` to open the popup"
    };

    let paragraph = Paragraph::new(text.slow_blink())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, vertical[1]);

    let block = Block::default()
        .title(" Popup Demo ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    f.render_widget(block, area);

    if app.popup_opened {
        let popup = tui_confirm_dialog::PopupMessage::new(
            " Loading ",
            vec![
                Line::from("Example popup showing a loading message"),
                Line::from("The operation was successful"),
            ],
        )
        .title_alignment(Alignment::Center)
        .text_alignment(Alignment::Center)
        .text_style(Style::new().black())
        .bg(Color::Yellow)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(Color::Red));
        f.render_widget(popup, area);
    }
}
