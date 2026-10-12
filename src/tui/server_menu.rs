use std::sync::Arc;
use parking_lot::RwLock;

use crate::network::{ServerList, Server};
use crate::tui::{Component, Label, block_and_inner};

use ratatui::prelude::{Text, Rect, Buffer};
use ratatui::widgets::{List, ListState, StatefulWidget};
use crossterm::event::KeyCode;

pub struct ServerMenu {
    server_list: Arc<RwLock<ServerList>>,
    scroll_state: ListState,
}

impl ServerMenu {
    pub fn new(server_list: Arc<RwLock<ServerList>>) -> Self {
        let scroll_state = ListState::default().with_selected(server_list.read().default);
        Self { server_list, scroll_state }
    }
}

impl Component for ServerMenu {
    fn render(&mut self, area: Rect, buf: &mut Buffer, selected: bool) {
        let server_list_lock = self.server_list.read();
        let list = List::new(&server_list_lock.servers);

        let inner = block_and_inner(area, buf);
        list.render(inner, buf, &mut self.scroll_state);
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Self::Action, anyhow::Error> {
        match key.code {
            KeyCode::Char('j') => self.scroll_state.select_next(),
            KeyCode::Char('k') => self.scroll_state.select_previous(),
            _ => {}
        }
        Ok(())
    }

    fn get_labels(&self) -> Vec<String> {
        vec![
            Label::new("j", "down"),
            Label::new("k", "up"),
            Label::new("x", "remove"),
            // This could be a non-render if you're ON the selected server...
            Label::new("enter", "select"),
        ]
    }
}

impl<'a> From<&Server> for Text<'a> {
    fn from(value: &Server) -> Self {
        Self::from(value.user_string())
    }
}
