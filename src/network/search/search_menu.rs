use crate::network::SearchResultsMenu;
use anyhow::Error;
use std::sync::Arc;
use parking_lot::RwLock;
use ratatui::layout::{ Layout, Direction, Constraint };
use crate::{network::ServerList, tui::{FramedWindow, Label, Window}};
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
    results: SearchResultsMenu,
    search_bar: SearchBar,
    mode: SearchMenuMode,
    server_list: Arc<RwLock<ServerList>>
}

// TODO: it should *create* a search menu and prompt *it* to search...
impl SearchMenu {
    pub fn new(query: String, results: SearchResultsMenu, server_list: Arc<RwLock<ServerList>>) -> Self {
        Self {results, search_bar: SearchBar::new(query), mode: SearchMenuMode::Results, server_list}
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
                self.results.handle_key_event(key)?
            }
            SearchMenuMode::Search => {
                if let KeyCode::Char('j') = key.code && key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.mode = SearchMenuMode::Results;
                    return Ok(())
                }

                if let Some(text) = self.search_bar.handle_key_event(key)? {
                    let server_list = self.server_list.read();
                    let results = server_list.search(server_list.get_default()?, text)?;
                    self.results = SearchResultsMenu::new(SearchResults::new(results, self.server_list.clone()));
                    self.mode = SearchMenuMode::Results
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
            (&mut self.search_bar).render_unselected(top, buf, "ctrl+k");
            (&mut self.results).render(bottom, buf);
        } else {
            (&mut self.search_bar).render_selected(top, buf, &mut vec![Label::new("enter", "search")]);
            (&mut self.results).render_greyed_out(bottom, buf, "ctrl+j");
        }
        return
    }
}