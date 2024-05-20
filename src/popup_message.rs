use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::style::Stylize;
use ratatui::text::Text;
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Widget};

use crate::PopupMessage;

impl<'a, 'b> PopupMessage<'a, 'b> {
	pub fn new<T, M>(title: T, message: M) -> Self
	where
		T: Into<Title<'a>>,
		M: Into<Text<'b>>,
	{
		PopupMessage {
			bg: Default::default(),
			borders: Default::default(),
			border_type: Default::default(),
			border_style: Default::default(),
			title_alignment: Default::default(),
			text_alignment: Default::default(),
			text_style: Default::default(),
			padding: Padding::uniform(2),
			title: title.into(),
			message: message.into(),
		}
	}
}

impl PopupMessage<'_, '_> {
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

	pub fn text_alignment(mut self, alignment: Alignment) -> Self {
		self.text_alignment = alignment;
		self
	}

	pub fn title_alignment(mut self, alignment: Alignment) -> Self {
		self.title_alignment = alignment;
		self
	}

	pub fn text_style(mut self, style: Style) -> Self {
		self.text_style = style;
		self
	}

	pub fn padding(mut self, padding: Padding) -> Self {
		self.padding = padding;
		self
	}
}

impl Widget for PopupMessage<'_, '_> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let horizontal_padding = self.padding.left + self.padding.right;
		let vertical_padding = self.padding.top + self.padding.bottom;

		let block = Block::default()
			.title(self.title)
			.title_alignment(self.title_alignment)
			.borders(self.borders)
			.border_type(self.border_type)
			.border_style(self.border_style)
			.padding(self.padding)
			.bg(self.bg);

		let mut width = self
			.message
			.lines
			.iter()
			.max_by(|a, b| a.width().cmp(&b.width()))
			.map(|line| line.width() + (horizontal_padding as usize) + 2)
			.unwrap() as u16;

		if width % 2 == 1 {
			width = width.saturating_add(1);
		}

		let mut height = (self.message.lines.len() + vertical_padding as usize) as u16 + 1;
		if height % 2 == 1 {
			height = height.saturating_add(1);
		}

		let paragraph = Paragraph::new(self.message)
			.style(self.text_style)
			.alignment(self.text_alignment)
			.block(block);

		let centered_area = super::helper::centered_rect_with_size(width, height + 1, area);

		Clear::default().render(centered_area, buf);
		paragraph.render(centered_area, buf);
	}
}
