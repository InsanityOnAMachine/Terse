use std::sync::Arc;
use parking_lot::RwLock;

use ratatui::widgets::{ListState};
use ratatui::prelude::{Widget, Rect, Buffer, Text};
use crossterm::event::KeyCode;

use crate::network::{Server, ServerList};
use crate::tui::{self, Window, Label};

pub struct SearchServerSelector {
	server_list: Arc<RwLock<ServerList>>,
	state: ListState,
}

impl SearchServerSelector {
	pub fn new(server_list: Arc<RwLock<ServerList>>) -> Self {
		let default = server_list.read().default;
		Self {server_list, state: ListState::default().with_selected(default)}
	}

	pub fn current_server(&self) -> Server {
		self.server_list.read().get_server(self.state.selected().expect("Something should always be selected")).unwrap().clone()
	}

	pub fn get_desired_width(&self) {
		todo!("Get desired width and grapheme clusters and all that")
	}
}

impl Window for SearchServerSelector {
	fn render(&mut self, area: Rect, buf: &mut Buffer) {

            let block = tui::get_default_block();
            let inner = block.inner(area);
            block.render(area, buf);

            Text::from(self.server_list.read().get_server(self.state.selected().expect("Something should always be selected")).unwrap().as_string(false, false)).render(inner, buf);
        }

	fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Self::Action, anyhow::Error> {
		match key.code {
                    KeyCode::Char('j') => {self.state.select_next();}
                    KeyCode::Char('k') => {self.state.select_previous();}
			_ => {}
		}
            // https://doc.rust-lang.org/std/primitive.i32.html#method.rem_euclid
            self.state.select(self.state.selected().map(|x| x.rem_euclid(self.server_list.read().servers.len())));
	    Ok(())
	}
    
    fn get_labels() -> Vec<String> {
        return vec![
            Label::new("j", "next"),
            Label::new("k", "prev"),
        ]
    }
}
