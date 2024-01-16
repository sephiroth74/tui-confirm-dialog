use std::sync::mpsc::Sender;

#[cfg(feature = "crossterm")]
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use rand::random;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::prelude::{Direction, Style, Text};
use ratatui::style::{Color, Stylize};
use ratatui::text::Span;
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, StatefulWidget, Widget, Wrap};

use crate::{ButtonLabel, ConfirmDialog, ConfirmDialogState, Listener};

impl ButtonLabel {
	pub fn new<S>(label: S, control: char) -> Self
	where
		S: Into<String>,
	{
		ButtonLabel {
			label: label.into(),
			control,
		}
	}

	pub fn len(&self) -> usize {
		self.label.len()
	}
}

impl ConfirmDialogState {
	pub fn new<T, R>(id: u16, title: T, text: R) -> Self
	where
		T: Into<Title<'static>>,
		R: Into<Text<'static>>,
	{
		ConfirmDialogState {
			id,
			title: title.into(),
			text: text.into(),
			modal: false,
			opened: false,
			listener: None,
			yes_button: ButtonLabel::new("Yes", 'y'),
			no_button: Some(ButtonLabel::new("No", 'n')),
		}
	}

	pub fn open(mut self) -> Self {
		self.opened = true;
		self
	}

	pub fn close(mut self) -> Self {
		self.opened = false;
		self
	}

	pub fn modal(mut self, value: bool) -> Self {
		self.modal = value;
		self
	}

	pub fn with_title<T>(mut self, title: T) -> Self
	where
		T: Into<Title<'static>>,
	{
		self.title = title.into();
		self
	}

	pub fn with_text<T>(mut self, text: T) -> Self
	where
		T: Into<Text<'static>>,
	{
		self.text = text.into();
		self
	}

	pub fn with_listener(mut self, listener: Option<Sender<Listener>>) -> Self {
		self.listener = listener;
		self
	}

	pub fn with_yes_button(mut self, label: ButtonLabel) -> Self {
		self.yes_button = label;
		self
	}

	pub fn with_no_button(mut self, label: ButtonLabel) -> Self {
		self.no_button = Some(label);
		self
	}

	pub fn without_no_button(mut self) -> Self {
		self.no_button = None;
		self
	}

	pub fn is_opened(&self) -> bool {
		self.opened
	}

	#[cfg(feature = "crossterm")]
	pub fn handle(&mut self, event: KeyEvent) -> bool {
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

impl ConfirmDialog {
	pub fn new() -> Self {
		ConfirmDialog {
			bg: Default::default(),
			borders: Default::default(),
			border_type: Default::default(),
			border_style: Default::default(),
		}
	}

	pub fn bg(mut self, color: Color) -> Self {
		self.bg = color;
		self
	}

	pub fn borders(mut self, borders: Borders) -> Self {
		self.borders = borders;
		self
	}

	pub fn border_type(mut self, border_type: BorderType) -> Self {
		self.border_type = border_type;
		self
	}

	pub fn border_style(mut self, border_style: Style) -> Self {
		self.border_style = border_style;
		self
	}

	fn button_paragraph(button: &ButtonLabel) -> Paragraph {
		let label = &button.label;
		let control_char = button.control;

		if label.len() > 1 {
			let control_char = control_char.to_string().to_lowercase();
			let splitted = label.to_lowercase().find(&control_char).unwrap();

			let mut spans = vec![];
			let (first, second) = label.split_at(splitted);

			if first.len() > 0 {
				spans.push(Span::styled(first, Style::new().yellow()));
			}

			if second.len() == 1 {
				spans.push(Span::styled(second, Style::new().yellow().underlined()));
			} else {
				let (first, second) = second.split_at(1);
				spans.push(Span::styled(first, Style::new().yellow().underlined()));
				spans.push(Span::styled(second, Style::new().yellow()));
			}
			Paragraph::new(ratatui::text::Line::from(spans))
		} else {
			Paragraph::new(Span::styled(label, Style::new().yellow().underlined()))
		}
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

		let yes_button = Self::button_paragraph(&state.yes_button);
		let yes_button_size = (state.yes_button.len() + buttons_padding as usize) as u16;
		let mut no_button_size = 0u16;

		let no_button = if let Some(no_button) = &state.no_button {
			no_button_size = (no_button.len() + buttons_padding as usize) as u16;
			Some(Self::button_paragraph(no_button))
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
			.style(Style::new().white())
			.wrap(Wrap { trim: true });

		let centered_area = super::helper::centered_rect_with_size(width, height, area);

		let main_layout = Layout::new()
			.direction(Direction::Vertical)
			.constraints(&[
				Constraint::Min(1),
				Constraint::Max(2),
			])
			.split(centered_area);

		Clear::default().render(centered_area, buf);
		text_widget.render(main_layout[0], buf);
		block.render(centered_area, buf);

		let buttons_layout = Layout::new().direction(Direction::Horizontal);

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
