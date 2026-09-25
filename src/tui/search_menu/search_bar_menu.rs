use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::layout::{Layout, Direction, Constraint};
use ratatui::prelude::{Rect, Buffer};

use crate::network::Server;
use crate::tui::Component;

use super::{SearchBar, SearchServerSelector};

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
    /// I opt to make this take in the components as arguments instead of stuff to build the
    /// components because it is less coupled, if we change the arguments that the components are
    /// made of. Still, both ways have their pros and cons. 
    pub fn new(search_bar: SearchBar, server_selector: SearchServerSelector) -> Self {
        Self {search_bar, server_selector, mode: SearchBarMenuMode::Search}
    }
}

impl Component for SearchBarMenu {
    type Action = SearchBarMenuAction;
    fn render(&mut self, area: Rect, buf: &mut Buffer, selected: bool) {
        let [left, right] = Layout::new(Direction::Horizontal, vec![
            Constraint::Fill(1),
            Constraint::Min(self.server_selector.get_min_size().width)
        ]).areas(area);

        match self.mode {
            SearchBarMenuMode::Search => {
                if selected {
                    self.search_bar.render_with_help(left, buf);
                } else {
                    self.search_bar.render(left, buf, selected);
                }
                self.server_selector.render_greyed_out(right, buf, if selected {"ctrl-l"} else {""});
            },
            SearchBarMenuMode::Server => {
                self.search_bar.render_greyed_out(left, buf, if selected {"ctrl-h"} else {""});
                self.server_selector.render_with_help(right, buf);
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

    fn deselect(&mut self) {
        self.mode = SearchBarMenuMode::Search;
        self.search_bar.deselect();
        self.server_selector.deselect();
    }
    fn get_min_size(&self) -> crate::tui::MinSize {
        crate::tui::MinSize::hmerge(vec![self.search_bar.get_min_size(), self.server_selector.get_min_size()])
    }
}
