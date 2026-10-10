use ratatui::prelude::Widget;
use std::collections::HashMap;
use ratatui::layout::{ Layout, Direction, Constraint };
use std::sync::Arc;
use parking_lot::RwLock;
use crate::tui::{self, Label, Component, PostReader};
use crate::network::{NetworkError, SearchResult, ServerList};
use crate::posts::Post;

use ratatui::widgets::{StatefulWidget, List, ListState};
use ratatui::prelude::{Rect, Buffer, Modifier};

use crossterm::event::{KeyCode, KeyEvent};

pub struct SearchResults {
    links: Vec<SearchResult>,
    post_cache: HashMap<SearchResult, Post>,
    list_state: ListState,
    server_list: Arc<RwLock<ServerList>>,
}

impl SearchResults{
    pub fn new(links: Vec<SearchResult>, server_list: Arc<RwLock<ServerList>>) -> Self {
        let mut list_state = ListState::default();
        list_state.select_first();

        let post_cache = HashMap::with_capacity(links.len());

        Self { links, list_state, post_cache, server_list}
    }

    /// Attempts to get a Post from the selected SearchResult
    ///
    /// If there are no results or for some reason none are selected, it returns Ok(None)
    /// If there is a network error trying to get the post, it returns Err(NetworkError)
    /// If everything goes well, you'll get an Ok(Post)
    ///
    pub fn get_selected_article(&mut self) -> Result<Option<Post>, NetworkError> {
        // https://stackoverflow.com/questions/37890405/is-there-a-way-to-simplify-converting-an-option-into-a-result-without-a-macro
        let selected_opt = self.list_state.selected();
        let selected = match selected_opt {
            Some(x) => x,
            None => return Ok(None)
        };

        let search_result_opt = self.links.get(selected.min(self.links.len()-1));

        let search_result = match search_result_opt {
            Some(x) => x,
            None => return Ok(None)
        };

        if !self.post_cache.contains_key(search_result) {
            let server_list = self.server_list.read();
            let post = server_list.get_post(&search_result.server, search_result.header.postid)?;
            self.post_cache.insert(search_result.clone(), post);
        }
        return Ok(Some(self.post_cache.get(search_result).unwrap().clone()))
    }

    pub fn has_selected_article(&self) -> bool {
        let search_result = self.links.get(self.list_state.selected().unwrap_or(0).min(self.links.len()-1));
        search_result.map_or(false, |search_result: &SearchResult| self.post_cache.contains_key(search_result))
    }

    pub fn get_width(&self) -> usize {
        // https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or
        return self.links.iter().map(|x| x.header.title.len()).max().unwrap_or(10)
    }
}

pub enum SearchResultsAction {
    Nothin,
    Moved,
    Selected,
}

impl Component for SearchResults {
    type Action = SearchResultsAction;
    
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Self::Action, anyhow::Error> {
        Ok(match key.code {
            KeyCode::Char('j') => {self.list_state.select_next(); Self::Action::Moved}
            KeyCode::Char('k') => {self.list_state.select_previous(); Self::Action::Moved}
            KeyCode::Enter => Self::Action::Selected,
            _ => {Self::Action::Nothin}
        })
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, _selected: bool) {
        let block = tui::get_default_block();
        let inner = block.inner(area);
        block.render(area, buf);

        // The actual results part
        StatefulWidget::render(
            List::new(self.links.iter().map(|x| &x.header))
                // This can also be a style.
                .highlight_style(Modifier::REVERSED)
                .scroll_padding(3),
            inner,
            buf,
            &mut self.list_state,
        );
    }
    
    fn get_labels(&self) -> Vec<String> {
        return vec![
            Label::new("j", "down"),
            Label::new("k", "up"),
            Label::new("enter", "select"),
        ]
    }
    fn get_title(&self) -> &str {
        "Search Results"
    }
    fn get_min_size(&self) -> tui::MinSize {
        tui::MinSize::new(self.get_text_fit_size() as u16, 5)
    }
}


pub struct SearchResultsMenu {
    search_results: SearchResults,
    post_reader: Result<Option<PostReader>, NetworkError>,
    mode: SearchResultsMenuMode,
}

impl SearchResultsMenu {
    pub fn new(search_results: SearchResults) -> Self {
        Self {search_results, post_reader: Ok(None), mode: SearchResultsMenuMode::Results}
    }
}

pub enum SearchResultsMenuMode {
    Results,
    Post,
}

impl Component for SearchResultsMenu {
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<(), anyhow::Error> {
        match self.mode {
            SearchResultsMenuMode::Results => {
                match self.search_results.handle_key_event(key)? {
                    SearchResultsAction::Moved => {
                        if self.search_results.has_selected_article() {
                            self.post_reader = self.search_results
                                .get_selected_article()
                                .map(|x| x.map(|y| PostReader::new(y)))
                        } else {
                            self.post_reader = Ok(None)
                        }
                    },
                    SearchResultsAction::Selected => {
                        // This means that even if we are showing the preview, we make a new post
                        // widget anyway. Also means that on deselect for the PostReader we need to
                        // wind it back up to the top so there's no jump as the new one always
                        // starts out at 0 scroll
                        self.post_reader = self.search_results.get_selected_article().map(|x| x.map(|y| PostReader::new(y)));
                        self.mode = SearchResultsMenuMode::Post;
                    },
                    _ => {}
                }
            },
            SearchResultsMenuMode::Post => {
                match key.code {
                    KeyCode::Char('b') => {
                        self.mode = SearchResultsMenuMode::Results;
                        self.post_reader.deselect();
                    }
                    _ => _ = self.post_reader.handle_key_event(key)?
                }
            }
        }
        Ok(())
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, selected: bool) {
        let [left, right] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(std::cmp::max(self.search_results.get_width() as u16 + 8, 35)),
                Constraint::Fill(1),
            ])
            .areas(area);

        match self.mode {
            SearchResultsMenuMode::Results => {
                self.search_results.render_with_help(left, buf);
                self.post_reader.render_greyed_out(right, buf, "");
            }
            SearchResultsMenuMode::Post => {
                self.search_results.render_greyed_out(left, buf, if selected {"b"} else {""});
                self.post_reader.render_with_help(right, buf) 
            }
        }
    }

    fn deselect(&mut self) {
        self.mode = SearchResultsMenuMode::Results;
        self.post_reader.deselect();
    }
    fn get_min_size(&self) -> tui::MinSize {
        tui::MinSize::hmerge(vec![self.search_results.get_min_size(), self.post_reader.get_min_size()])
    }
}
