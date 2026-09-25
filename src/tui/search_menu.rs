pub mod search_bar;
pub use search_bar::*;

pub mod search_results;
pub use search_results::*;

pub mod search_bar_menu;
pub use search_bar_menu::*;

pub mod search_server_select;
pub use search_server_select::*;


use std::sync::Arc;
use parking_lot::RwLock;

use ratatui::layout::{ Layout, Direction, Constraint };
use crate::{network::{ServerList, Server}, tui::Component};

use anyhow::Error;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub enum SearchMenuMode {
    Results,
    Search,
}

pub struct SearchMenu {
    results: Result<SearchResultsMenu, Error>,
    search_menu: SearchBarMenu,
    mode: SearchMenuMode,
    server_list: Arc<RwLock<ServerList>>
}

impl SearchMenu {
    pub fn new(query: String, server_list: Arc<RwLock<ServerList>>) -> Self {
        let mut menu = Self {
            results: Err(Error::msg("WAITING AND THIS SHOULD BE A SPECIAL TYPE...")),
            search_menu: SearchBarMenu::new(
                SearchBar::new(query.clone()),
                SearchServerSelector::new(server_list.clone()),
            ),
            mode: SearchMenuMode::Search,
            server_list: server_list.clone()
        };
        menu.process_search(query, server_list.read().get_default().unwrap());
        menu
    }

    fn process_search(&mut self, query: String, server: &Server) {
        let server_list = self.server_list.read();
        self.results = || -> Result<SearchResultsMenu, Error> {
            let results = server_list.search(server, query)?;
            if results.is_empty() {return Err(Error::msg("No results matching that query!"))}
            return Ok(SearchResultsMenu::new(SearchResults::new(results, self.server_list.clone())))
        }();
        if self.results.is_ok() {self.mode = SearchMenuMode::Results}
    }
}


impl Component for SearchMenu {
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<(), Error> {
        match &self.mode {
            SearchMenuMode::Results => {
                if let KeyCode::Char('k') = key.code && key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.mode = SearchMenuMode::Search;
                    self.results.deselect();
                    return Ok(())
                }
                self.results.handle_key_event(key)?;
            }
            SearchMenuMode::Search => {
                if let KeyCode::Char('j') = key.code && key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.mode = SearchMenuMode::Results;
                    self.search_menu.deselect();
                    return Ok(())
                }

                if let SearchBarMenuAction::Search { query, server } = self.search_menu.handle_key_event(key)? {
                    self.process_search(query, &server);
                }
            }
        }

        Ok(())
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, selected: bool) {
        let [top, bottom] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .areas(area);

        // We render the search menu second 'cause its dropdown needs to be on top when opened
        if let SearchMenuMode::Results = &mut self.mode {
            (&mut self.results).render(bottom, buf, selected);
            (&mut self.search_menu).render_greyed_out(top, buf, if selected {"ctrl+k"} else {""});
        } else {
            (&mut self.results).render_greyed_out(bottom, buf, if selected {"ctrl+j"} else {""});
            (&mut self.search_menu).render_with_help(top, buf);
        }
        return
    }

    fn get_min_size(&self) -> super::MinSize {
        super::MinSize::vmerge(vec![self.search_menu.get_min_size(), self.results.get_min_size()])
    }
}
