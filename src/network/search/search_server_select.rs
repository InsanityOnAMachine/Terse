use std::sync::Arc;
use parking_lot::RwLock;

use ratatui::widgets::{StatefulWidget, List, ListState, Clear};
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

	pub fn current_server(&self) -> Server {
		self.server_list.read().get_server(self.state.selected().expect("Something should always be selected")).unwrap().clone()
	}
}

impl Window for SearchServerSelector {
	fn render(&mut self, area: Rect, buf: &mut Buffer) {

		let block = tui::get_default_block();

		if self.open {
			let server_list_lock = self.server_list.read();

			// +2 to account for the block top and bottom
			let render_area = Rect::new(area.x, area.y, area.width, (buf.area().height - area.y).min(server_list_lock.servers.len() as u16) + 2);
			let inner = block.inner(render_area);
	    	block.render(render_area, buf);

	    	Clear.render(inner, buf);
	    	let list = List::from(server_list_lock.servers.iter().map(|x| x.url_string()).collect());
	    	StatefulWidget::render(list, inner, buf, &mut self.state);
		} else {
		    let inner = block.inner(area);
		    block.render(area, buf);

	    	Text::from(self.server_list.read().get_server(self.state.selected().expect("Something should always be selected")).unwrap().as_string(false, false)).render(inner, buf);
	    	Text::from(" v").right_aligned().render(inner, buf);
	    }
	}

	fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Self::Action, anyhow::Error> {
		match key.code {
			KeyCode::Enter => {self.open = !self.open}
			KeyCode::Char('j') => {self.state.select_next();}
			KeyCode::Char('k') => {self.state.select_previous();}
			_ => {}
		}
	    Ok(())
	}

	fn deselect(&mut self) {
	    self.open = false
	}

}