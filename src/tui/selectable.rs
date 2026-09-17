use anyhow::Error;
use ratatui::widgets::{StatefulWidget, List, ListState};
use ratatui::prelude::{Rect, Buffer, Text};
use ratatui::style::Modifier;

use crossterm::event::{KeyCode, KeyEvent};

use crate::tui::*;

pub struct Selectable<T> where for<'a> Text<'a>: From<&'a T> {
    items: Vec<T>,
    pub state: ListState,
}

impl<T> Selectable<T> where for<'a> Text<'a>: From<&'a T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {items, state: ListState::default().with_selected(Some(0))}
    }

    pub fn selected_item(&self) -> Option<&T> {
        self.items.get(self.state.selected()?)
    }
}

impl <T> Window for &mut Selectable<T> where for<'a> Text<'a>: From<&'a T> {
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<(), Error> {
        match key.code {
            // TODO: clamp after!
            KeyCode::Char('j') => self.state.scroll_down_by(1),
            KeyCode::Char('k') => self.state.scroll_up_by(1),
            _ => (),
        }
        Ok(())
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, _selected: bool) {
        let container = get_default_block()
            .title_bottom("( (j / k) + enter to select )");

        StatefulWidget::render(
            List::new(&self.items)
                // This can also be a style.
                .highlight_style(Modifier::REVERSED)
                .scroll_padding(3)
                .block(container),
            area,
            buf,
            &mut self.state,
        );
    }
}
