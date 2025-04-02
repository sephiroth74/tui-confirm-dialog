use core::fmt;
use std::error::Error;
use std::fmt::Display;
use std::sync::mpsc::Sender;
use std::sync::LazyLock;

#[cfg(feature = "crossterm")]
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use rand::random;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::prelude::{Direction, Style, Text};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::block::Title;
use ratatui::widgets::{
    Block, BorderType, Borders, Clear, Padding, Paragraph, StatefulWidget, Widget, Wrap,
};
use regex::Regex;

use crate::{ButtonLabel, ConfirmDialog, ConfirmDialogState, Listener, TryFromSliceError};

static BUTTON_LABEL_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(\(\w\))"#).unwrap());

impl ButtonLabel {
    pub const YES: LazyLock<ButtonLabel> = LazyLock::new(|| ButtonLabel {
        label: "(Y)es".to_string(),
        control: 'y',
        style: None,
    });

    pub const NO: LazyLock<ButtonLabel> = LazyLock::new(|| ButtonLabel {
        label: "(N)o".to_string(),
        control: 'n',
        style: None,
    });

    pub fn new<S>(label: S, control: char) -> Self
    where
        S: Into<String>,
    {
        ButtonLabel {
            label: label.into(),
            control,
            style: None,
        }
    }

    pub fn from<S>(label: S) -> Result<Self, TryFromSliceError>
    where
        S: Into<String>,
    {
        label.into().as_str().try_into()
    }

    pub fn len(&self) -> usize {
        self.label.len() + 2
    }

    pub(crate) fn with_style(mut self, style: Option<Style>) -> Self {
        self.style = style;
        self
    }
}

impl TryFrom<&str> for ButtonLabel {
    type Error = TryFromSliceError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(TryFromSliceError);
        }

        if let Some(result) = BUTTON_LABEL_RE.find(value) {
            let control_char = value.chars().nth(result.start() + 1).unwrap();
            Ok(ButtonLabel {
                label: value.to_string(),
                control: control_char.to_ascii_lowercase(),
                style: None,
            })
        } else {
            let char = value.chars().nth(0).unwrap();
            let label = format!("({char}){}", &value[1..]);

            Ok(ButtonLabel {
                label,
                control: char.to_ascii_lowercase(),
                style: None,
            })
        }
    }
}

impl<'a> Into<Text<'a>> for ButtonLabel {
    fn into(self) -> Text<'a> {
        Text::styled(self.label, self.style.unwrap_or(Style::default()))
    }
}

impl PartialEq for ButtonLabel {
    fn eq(&self, other: &Self) -> bool {
        eprintln!("self: {:?}, other: {:?}", self, other);
        self.control == other.control && self.label == other.label
    }
}

impl ConfirmDialogState {
    pub fn new<T, R>(id: u16, title: T, text: R) -> Self
    where
        T: Into<Title<'static>>,
        R: Into<Text<'static>>,
    {
        let yes_button = ButtonLabel::new("Yes", 'y');
        let no_button = ButtonLabel::new("No", 'n');

        ConfirmDialogState {
            id,
            title: title.into(),
            text: text.into(),
            modal: false,
            opened: false,
            listener: None,
            yes_selected: true,
            yes_button,
            no_button: Some(no_button),
        }
    }

    pub fn as_mut(&mut self) -> &mut Self {
        self
    }

    /// Open the dialog
    pub const fn open(&mut self) {
        self.opened = true;
    }

    /// Close the dialog
    pub const fn close(&mut self) {
        self.opened = false;
    }

    /// Set the dialog as modal
    pub const fn modal(&mut self, value: bool) -> &mut Self {
        self.modal = value;
        self
    }

    /// Set the dialog title
    pub fn with_title<T>(&mut self, title: T) -> &mut Self
    where
        T: Into<Title<'static>>,
    {
        self.title = title.into();
        self
    }

    /// Set the dialog message
    pub fn with_text<T>(&mut self, text: T) -> &mut Self
    where
        T: Into<Text<'static>>,
    {
        self.text = text.into();
        self
    }

    /// Set the dialog listener
    pub fn with_listener(&mut self, listener: Option<Sender<Listener>>) -> &mut Self {
        self.listener = listener;
        self
    }

    /// Set the dialog `yes` button
    pub fn with_yes_button<B>(&mut self, label: B) -> &mut Self
    where
        B: Into<ButtonLabel>,
    {
        self.yes_button = label.into();
        self
    }

    /// Set the dialog `no` button
    pub fn with_no_button<B>(&mut self, label: B) -> &mut Self
    where
        B: Into<ButtonLabel>,
    {
        self.no_button = Some(label.into());
        self
    }

    /// Remove the `no` button
    pub fn without_no_button(&mut self) -> &mut Self {
        self.no_button = None;
        self
    }

    /// Set the selected button to Yes or No
    pub const fn with_yes_button_selected(&mut self, selected: bool) -> &mut Self {
        self.yes_selected = selected;
        self
    }

    /// Check if the dialog is opened
    pub fn is_opened(&self) -> bool {
        self.opened
    }

    #[cfg(feature = "crossterm")]
    /// Handle the dialog events
    pub fn handle(&mut self, event: &KeyEvent) -> bool {
        if event.kind == KeyEventKind::Press {
            match event.code {
                KeyCode::Esc => {
                    if !self.modal {
                        self.opened = false;
                        self.send_close_message(None);
                        true
                    } else {
                        false
                    }
                }

                KeyCode::Char(chr) => {
                    if chr == self.yes_button.control {
                        self.opened = false;
                        self.send_close_message(Some(true));
                        return true;
                    }

                    if let Some(no_button) = &self.no_button {
                        if chr == no_button.control {
                            self.opened = false;
                            self.send_close_message(Some(false));
                            return true;
                        }
                    }
                    self.modal
                }

                KeyCode::Right => {
                    if let Some(_no_button) = &self.no_button {
                        if self.yes_selected {
                            self.yes_selected = false;
                        }
                    }
                    self.modal
                }

                KeyCode::Left => {
                    if let Some(_no_button) = &self.no_button {
                        if !self.yes_selected {
                            self.yes_selected = true;
                        }
                    }
                    self.modal
                }

                KeyCode::Enter => {
                    if self.yes_selected {
                        self.opened = false;
                        self.send_close_message(Some(true));
                    } else {
                        self.opened = false;
                        self.send_close_message(Some(false));
                    }
                    true
                }

                _ => self.modal,
            }
        } else {
            false
        }
    }

    fn send_close_message(&self, result: Option<bool>) {
        if let Some(tx) = self.listener.as_ref() {
            let _ = tx.send((self.id, result));
        }
    }
}

impl Default for ConfirmDialog {
    fn default() -> Self {
        ConfirmDialog::new()
    }
}

impl ConfirmDialog {
    pub fn new() -> Self {
        ConfirmDialog {
            bg: Default::default(),
            borders: Default::default(),
            border_type: Default::default(),
            border_style: Default::default(),
            button_style: Style::new(),
            selected_button_style: Style::new().underlined(),
            text_style: Style::new().white(),
        }
    }

    /// Set the dialog background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    /// Set the dialog borders
    pub fn borders(mut self, borders: Borders) -> Self {
        self.borders = borders;
        self
    }

    /// Set the dialog border type
    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }

    /// Set the dialog border style
    pub fn border_style(mut self, border_style: Style) -> Self {
        self.border_style = border_style;
        self
    }

    /// Set the dialog button style
    pub fn button_style(mut self, button_style: Style) -> Self {
        self.button_style = button_style;
        self
    }

    /// Set the dialog selected button style
    pub fn selected_button_style(mut self, selected_button_style: Style) -> Self {
        self.selected_button_style = selected_button_style;
        self
    }

    /// Set the dialog text style
    pub fn text_style(mut self, text_style: Style) -> Self {
        self.text_style = text_style;
        self
    }

    fn button_paragraph(button: &ButtonLabel, style: Style) -> Paragraph {
        Paragraph::new(button.clone().with_style(Some(style)))
    }
}

impl Default for ConfirmDialogState {
    fn default() -> Self {
        ConfirmDialogState::new(random(), Title::default(), Text::default())
    }
}

impl StatefulWidget for ConfirmDialog {
    type State = ConfirmDialogState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let horizontal_padding = 2u16;
        let vertical_padding = 2u16;
        let buttons_padding = 2u16;

        let block = Block::default()
            .title(state.title.clone())
            .title_alignment(Alignment::Center)
            .borders(self.borders)
            .border_type(self.border_type)
            .border_style(self.border_style)
            .bg(self.bg);

        let yes_selected = state.yes_selected || state.no_button.is_none();

        let yes_button = Self::button_paragraph(
            &state.yes_button,
            if yes_selected {
                self.selected_button_style.clone()
            } else {
                self.button_style.clone()
            },
        );
        let yes_button_size = (state.yes_button.len() + buttons_padding as usize) as u16;
        let mut no_button_size = 0u16;

        let no_button = if let Some(no_button) = &state.no_button {
            no_button_size = (no_button.len() + buttons_padding as usize) as u16;
            Some(Self::button_paragraph(
                no_button,
                if !yes_selected {
                    self.selected_button_style.clone()
                } else {
                    self.button_style.clone()
                },
            ))
        } else {
            None
        };

        let min_width: u16 = ((yes_button_size + no_button_size) + horizontal_padding * 2) as u16;
        let text = state.text.clone();

        let mut width = text
            .lines
            .iter()
            .max_by(|a, b| a.width().cmp(&b.width()))
            .map(|line| line.width() + horizontal_padding as usize * 2)
            .unwrap_or(min_width as usize) as u16;

        width = width.max(min_width).max(40);

        let height = (text.lines.len() + 1 + (vertical_padding as usize * 2 + 1)) as u16;

        let lines = Text::from(text);

        let text_widget = Paragraph::new(lines)
            .block(Block::new().padding(Padding::new(
                horizontal_padding,
                horizontal_padding,
                vertical_padding,
                vertical_padding,
            )))
            .style(self.text_style)
            .wrap(Wrap { trim: true });

        let centered_area = super::helper::centered_rect_with_size(width, height, area);

        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(&[Constraint::Min(1), Constraint::Max(2)])
            .split(centered_area);

        Clear::default().render(centered_area, buf);
        text_widget.render(main_layout[0], buf);
        block.render(centered_area, buf);

        let buttons_layout = Layout::default().direction(Direction::Horizontal);

        if let Some(no_button) = no_button {
            let c = (main_layout[1].width - (yes_button_size + no_button_size)) / 2; // 19

            let buttons_layout = buttons_layout
                .constraints(&[
                    Constraint::Length(c),
                    Constraint::Max(yes_button_size),
                    Constraint::Max(no_button_size),
                    Constraint::Length(c),
                ])
                .split(main_layout[1]);

            yes_button.render(buttons_layout[1], buf);
            no_button.render(buttons_layout[2], buf);
        } else {
            let c = (main_layout[1].width - yes_button_size) / 2;
            let buttons_layout = buttons_layout
                .constraints(&[
                    Constraint::Length(c),
                    Constraint::Max(yes_button_size),
                    Constraint::Length(c),
                ])
                .split(main_layout[1]);

            yes_button.render(buttons_layout[1], buf);
        }
    }
}

impl Display for TryFromSliceError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[allow(deprecated)]
        self.description().fmt(f)
    }
}

impl Error for TryFromSliceError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "could not convert slice to array"
    }
}
