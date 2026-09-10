use anyhow::Error;
use ratatui::text::Line;
use crate::tui::{FramedWindow, Window};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
};

use crossterm::event::{KeyCode, KeyEvent};

pub struct SearchBar {
	pub text: String,
}

impl SearchBar {
	pub fn new(query: String) -> Self {
		SearchBar {text: query}
	}
}

impl Window for SearchBar {
	type Action = Option<String>;
	fn handle_key_event(&mut self, key: KeyEvent) -> Result<Self::Action, Error> {
		match key.code {
            KeyCode::Char(c) => {
                self.text.push(c);
            }
            KeyCode::Backspace => {
                let _ = self.text.pop();
            }
            KeyCode::Enter => {
            	if !self.text.is_empty() {return Ok(Some(self.text.clone()))}
            }
            _ => {}
        }
        Ok(None)
	}

	fn render(&mut self, area: Rect, buf: &mut Buffer) {
		Line::from(self.text.as_str()).render(area, buf);
	}
}

impl FramedWindow for SearchBar {}