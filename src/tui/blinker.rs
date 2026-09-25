use std::iter;
use ratatui::prelude::Stylize;
use ratatui::layout::Rect;
use ratatui::buffer::Buffer;
use ratatui::widgets::Widget;
use ratatui::text::Line;

#[derive(Default)]
pub struct Blinker {
	last_state: bool
}

impl Widget for &mut Blinker {

	fn render(self, area: Rect, buf: &mut Buffer) {
		// https://doc.rust-lang.org/std/iter/fn.repeat.html
		let mut line = Line::from(iter::repeat(" ").take(area.width.into()).collect::<String>());
		if self.last_state {
			line = line.on_white()
		} else {
			line = line.on_red()
		}

		self.last_state = !self.last_state;

		line.render(area, buf);
	}
}
