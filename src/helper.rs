use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// helper function to create a centered rect using up certain percentage of the available rect `r`
#[allow(dead_code)]
pub(crate) fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
	let popup_layout = Layout::default()
		.direction(Direction::Vertical)
		.constraints([
			Constraint::Percentage((100 - percent_y) / 2),
			Constraint::Percentage(percent_y),
			Constraint::Percentage((100 - percent_y) / 2),
		])
		.split(r);

	Layout::default()
		.direction(Direction::Horizontal)
		.constraints([
			Constraint::Percentage((100 - percent_x) / 2),
			Constraint::Percentage(percent_x),
			Constraint::Percentage((100 - percent_x) / 2),
		])
		.split(popup_layout[1])[1]
}

#[allow(dead_code)]
pub(crate) fn centered_rect_with_size(width: u16, height: u16, r: Rect) -> Rect {
	let width = width.min(r.width);
	let height = height.min(r.height);
	let remaining_width = r.width.saturating_sub(width);
	let remaining_height = r.height.saturating_sub(height);

	let popup_layout = Layout::default()
		.direction(Direction::Vertical)
		.constraints([
			Constraint::Max(remaining_height / 2),
			Constraint::Length(height),
			Constraint::Max(remaining_height / 2),
		])
		.split(r);

	let final_area = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([
			Constraint::Max(remaining_width / 2),
			Constraint::Length(width),
			Constraint::Max(remaining_width / 2),
		])
		.split(popup_layout[1])[1];

	final_area
}
