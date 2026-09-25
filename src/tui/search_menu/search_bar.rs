use anyhow::Error;
use ratatui::text::Line;
use crate::tui::{self, Component, Label};

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

impl Component for SearchBar {
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

    fn render(&mut self, area: Rect, buf: &mut Buffer, _selected: bool) {
        let block = tui::get_default_block();
        let inner = block.inner(area);
        block.render(area, buf);

        Line::from(self.text.as_str()).render(inner, buf);
    }
    fn get_labels(&self) -> Vec<String> {
        return vec![
            Label::new("enter", "search")
        ]
    }
    fn get_min_size(&self) -> tui::MinSize {
        tui::MinSize::new((Label::join(self.get_labels()).len() + 2) as u16, 3)
    }
}
