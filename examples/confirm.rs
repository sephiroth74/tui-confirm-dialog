use std::{error::Error, io};

use ratatui::crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::text::{Line, Span};
use ratatui::{prelude::*, widgets::*};

use tui_confirm_dialog::{ButtonLabel, ConfirmDialog, ConfirmDialogState, Listener};

struct App {
    close_status: Option<String>,
    confirm_popup: ConfirmDialogState,
    popup_tx: std::sync::mpsc::Sender<Listener>,
    popup_rx: std::sync::mpsc::Receiver<Listener>,
}

impl App {
    fn new() -> App {
        let (tx, rx) = std::sync::mpsc::channel();
        App {
            confirm_popup: ConfirmDialogState::default(),
            popup_tx: tx,
            popup_rx: rx,
            close_status: None,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        if let Ok(res) = app.popup_rx.try_recv() {
            if res.0 == app.confirm_popup.id {
                app.close_status = Some(format!("Dialog closed with result: {:?}", res.1));
            }
        }

        terminal.draw(|f| ui(f, &mut app)).expect("panic message");

        if let Event::Key(key) = event::read()? {
            if app.confirm_popup.is_opened() && app.confirm_popup.handle(&key) {
                continue;
            }

            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('p') => {
                        app.confirm_popup
                            .modal(false)
                            .with_title(Span::styled(" Please Select ", Style::new().bold().cyan()))
                            .with_text(Text::from(vec![
                                Line::from("Are you sure you want to delete all files?"),
                                Line::from("This action cannot be undone."),
                            ]))
                            .with_yes_button(ButtonLabel::from("(Y)es").unwrap())
                            .with_no_button(ButtonLabel::NO.clone())
                            .with_yes_button_selected(false)
                            .with_listener(Some(app.popup_tx.clone()));
                        app.confirm_popup.open();
                    }
                    _ => {}
                }
            }
        }
    }
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

    let text = if app.confirm_popup.is_opened() {
        "Confirm dialog is being shown...".to_string()
    } else if let Some(close_status) = &app.close_status {
        close_status.clone()
    } else {
        "Press `p` to open the dialog".to_string()
    };

    let paragraph = Paragraph::new(text.slow_blink())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, vertical[1]);

    let block = Block::default().title(" Demo ").borders(Borders::ALL);
    f.render_widget(block, area);

    if app.confirm_popup.is_opened() {
        let popup = ConfirmDialog::default()
            .borders(Borders::ALL)
            .bg(Color::Black)
            .border_type(BorderType::Rounded)
            .button_style(Style::default())
            .selected_button_style(Style::default().yellow().underlined().bold());
        f.render_stateful_widget(popup, area, &mut app.confirm_popup);
    }
}
