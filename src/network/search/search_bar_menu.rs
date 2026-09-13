use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::layout::{Layout, Direction, Constraint};
use ratatui::prelude::{Rect, Buffer};

use crate::network::{Server, SearchBar, SearchServerSelector};
use crate::tui::{Window, Label};

pub struct SearchBarMenu {
	search_bar: SearchBar,
	server_selector: SearchServerSelector,
	mode: SearchBarMenuMode,
}

pub enum SearchBarMenuMode {
    Search,
    Server,
}

pub enum SearchBarMenuAction {
	Nothin,
	Search {query: String, server: Server}
}

impl SearchBarMenu {
	pub fn new(search_bar: SearchBar, server_selector: SearchServerSelector) -> Self {
		Self {search_bar, server_selector, mode: SearchBarMenuMode::Search}
	}
}

impl Window for SearchBarMenu {
	type Action = SearchBarMenuAction;
	fn render(&mut self, area: Rect, buf: &mut Buffer) {
		let [left, right] = Layout::new(Direction::Horizontal, vec![
			Constraint::Fill(1),
			Constraint::Length(25)
		]).areas(area);

		match self.mode {
			SearchBarMenuMode::Search => {
				self.search_bar.render_with_help(left, buf, &mut vec![Label::new("enter", "search")]);
				self.server_selector.render_greyed_out(right, buf, "ctrl-l");
			},
			SearchBarMenuMode::Server => {
				self.search_bar.render_greyed_out(left, buf, "ctrl-h");
				self.server_selector.render_with_help(right, buf, &mut vec![]);
			}
		}	    
	}

	fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Self::Action, anyhow::Error> {
	    match self.mode {
			SearchBarMenuMode::Search => {
				if let KeyCode::Char('l') = key.code && key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.mode = SearchBarMenuMode::Server;
                    return Ok(SearchBarMenuAction::Nothin)
                }

				if let Some(query) = self.search_bar.handle_key_event(key)? {
					return Ok(SearchBarMenuAction::Search { query, server: self.server_selector.current_server()})
				}
			},
			SearchBarMenuMode::Server => {
				if let KeyCode::Char('h') = key.code && key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.mode = SearchBarMenuMode::Search;
                    return Ok(SearchBarMenuAction::Nothin)
                }

				self.server_selector.handle_key_event(key)?;
			}
		}	
		Ok(SearchBarMenuAction::Nothin)
	}

}
