use crate::network::SearchResultsMenu;
use anyhow::Error;
use std::sync::Arc;
use parking_lot::RwLock;
use ratatui::layout::{ Layout, Direction, Constraint };
use crate::{network::ServerList, tui::{Label, Window}};
use super::{SearchResults, SearchBar};

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
    search_bar: SearchBar,
    mode: SearchMenuMode,
    server_list: Arc<RwLock<ServerList>>
}

// TODO: it should *create* a search menu and prompt *it* to search...
impl SearchMenu {
    pub fn new(query: String, server_list: Arc<RwLock<ServerList>>) -> Self {
        let mut menu = Self {results: Err(Error::msg("WAITING AND THIS SHOULD BE A SPECIAL TYPE...")), search_bar: SearchBar::new(query.clone()), mode: SearchMenuMode::Search, server_list};
        menu.process_search(query);
        menu
    }

    fn process_search(&mut self, query: String) {
        let server_list = self.server_list.read();
        self.results = || -> Result<SearchResultsMenu, Error> {
            let results = server_list.search(server_list.get_default()?, query)?;
            if results.is_empty() {return Err(Error::msg("No results matching that query!"))}
            return Ok(SearchResultsMenu::new(SearchResults::new(results, self.server_list.clone())))
        }();
        if self.results.is_ok() {self.mode = SearchMenuMode::Results}
    }
}


impl Window for SearchMenu {
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<(), Error> {
        match &self.mode {
            SearchMenuMode::Results => {
                if let KeyCode::Char('k') = key.code && key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.mode = SearchMenuMode::Search;
                    return Ok(())
                }
                self.results.handle_key_event(key)?;
            }
            SearchMenuMode::Search => {
                if let KeyCode::Char('j') = key.code && key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.mode = SearchMenuMode::Results;
                    return Ok(())
                }

                if let Some(text) = self.search_bar.handle_key_event(key)? {
                    self.process_search(text);
                }
            }
        }

        Ok(())
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let [top, bottom] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .areas(area);

        if let SearchMenuMode::Results = &mut self.mode {
            (&mut self.search_bar).render_greyed_out(top, buf, "ctrl+k");
            (&mut self.results).render(bottom, buf);
        } else {
            (&mut self.search_bar).render_with_help(top, buf, &mut vec![Label::new("enter", "search")]);
            (&mut self.results).render_greyed_out(bottom, buf, "ctrl+j");
        }
        return
    }
}