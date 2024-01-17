#![doc = include_str!("../README.md")]

use std::sync::mpsc::Sender;

use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::block::Title;
use ratatui::widgets::{BorderType, Borders};

pub mod helper;
mod impls;
mod test;

pub type Listener = (u16, Option<bool>);

#[derive(Debug, Copy, Clone)]
pub struct TryFromSliceError;

#[derive(Debug, Clone)]
pub struct ButtonLabel {
	pub(crate) label: String,
	pub(crate) control: char,
	pub(crate) style: Style,
}

#[derive(Debug)]
pub struct ConfirmDialogState {
	pub id: u16,
	pub(crate) title: Title<'static>,
	pub(crate) text: Text<'static>,
	pub(crate) modal: bool,
	pub(crate) opened: bool,
	pub(crate) yes_button: ButtonLabel,
	pub(crate) no_button: Option<ButtonLabel>,
	pub(crate) listener: Option<Sender<Listener>>,
}

#[derive(Debug, Default)]
pub struct ConfirmDialog {
	pub(crate) bg: Color,
	pub(crate) borders: Borders,
	pub(crate) border_type: BorderType,
	pub(crate) border_style: Style,
}
