use std::sync::Arc;
use parking_lot::RwLock;

use ratatui::widgets::ListState;
use ratatui::prelude::{Widget, Rect, Buffer, Text};
use crossterm::event::KeyCode;

use crate::network::{Server, ServerList};
use crate::tui::{self, Window};

pub struct SearchServerSelector {
	server_list: Arc<RwLock<ServerList>>,
	state: ListState,
	open: bool,
}

impl SearchServerSelector {
	pub fn new(server_list: Arc<RwLock<ServerList>>) -> Self {
		let default = server_list.read().default;
		Self {server_list, state: ListState::default().with_selected(default), open: false}
	}

	/// Opens up the menu's dropdown
	pub fn open(&mut self) {
		self.open = true;
	}

	/// Closes the dropdown; should be called when deselected
	pub fn close(&mut self) {
		self.open = false;
	}

	pub fn current_server(&self) -> Server {
		self.server_list.read().get_server(self.state.selected().expect("Something should always be selected")).unwrap().clone()
	}
}

impl Window for SearchServerSelector {
	fn render(&mut self, area: Rect, buf: &mut Buffer) {
	    let block = tui::get_default_block();
	    let inner = block.inner(area);
	    block.render(area, buf);

	    Text::from(self.server_list.read().get_server(self.state.selected().expect("Something should always be selected")).unwrap().as_string(false, false)).render(inner, buf);
	    Text::from(" v").right_aligned().render(inner, buf);
	}

	fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Self::Action, anyhow::Error> {
		match key.code {
			KeyCode::Enter => {
				if self.open {
					self.close();
				} else {
					self.open();
				}
			}
			KeyCode::Char('j') => {self.state.select_next();}
			KeyCode::Char('k') => {self.state.select_previous();}
			_ => {}
		}
	    Ok(())
	}

}