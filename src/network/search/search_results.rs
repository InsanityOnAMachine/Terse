use std::collections::HashMap;
use ratatui::layout::{ Layout, Direction, Constraint };
use std::sync::Arc;
use parking_lot::RwLock;
use crate::tui::{FramedWindow, Window, Label};
use crate::network::{Server, ServerList};
use crate::posts::{Post, PostWidget};

use ratatui::widgets::{StatefulWidget, List, ListState};
use ratatui::text::{Span, Line, Text};
use ratatui::prelude::{Rect, Buffer, Modifier};

use crossterm::event::{KeyCode, KeyEvent};

use serde::Deserialize;


#[derive(Deserialize, Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
pub struct SearchResultHeader {
    pub title: String,
    pub postid: u16,
}

#[derive(Eq, Hash, PartialEq)]
#[derive(Clone)]
pub struct SearchResult {
    pub header: SearchResultHeader,
    pub server: Server,
}


impl SearchResult {
    pub fn new(header: SearchResultHeader, server: Server) -> Self {
        Self {header, server}
    }
}

// https://www.reddit.com/r/rust/comments/7zm0j2/intofrom_for_nonconsuming_conversions/
impl<'a> From<&'a SearchResultHeader> for Text<'a> {
    fn from(value: &'a SearchResultHeader) -> Self {
        return Text::from(vec![Line::from(Span::from(&value.title))]);
    }
}

pub struct SearchResultsMenu {
    search_results: SearchResults,
    post_widget: Option<PostWidget>,
    mode: SearchResultsMenuMode,
}

impl SearchResultsMenu {
    pub fn new(search_results: SearchResults) -> Self {
        Self {search_results, post_widget: None, mode: SearchResultsMenuMode::Results}
    }
}

pub enum SearchResultsMenuMode {
    Results,
    Post,
}

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

    pub fn get_selected_article(&mut self) -> Post {
        // https://stackoverflow.com/questions/37890405/is-there-a-way-to-simplify-converting-an-option-into-a-result-without-a-macro
        let search_result = self.links.get(self.list_state.selected().unwrap_or(0)).unwrap();
        if !self.post_cache.contains_key(search_result) {
            let server_list = self.server_list.read();
            let post = server_list.get_post(&search_result.server, search_result.header.postid).unwrap();
            self.post_cache.insert(search_result.clone(), post);
        }
        return self.post_cache.get(search_result).unwrap().clone()
    }

    pub fn has_selected_article(&self) -> bool {
        let search_result = self.links.get(self.list_state.selected().unwrap_or(0));
        search_result.map_or(false, |search_result: &SearchResult| self.post_cache.contains_key(search_result))
    }

    pub fn get_width(&self) -> usize {
        // https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or
        return self.links.iter().map(|x| x.header.title.len()).max().unwrap_or(10)
    }
}

impl Window for SearchResultsMenu {
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<(), anyhow::Error> {
        match self.mode {
            SearchResultsMenuMode::Results => {
                match self.search_results.handle_key_event(key)? {
                    SearchResultsAction::Moved => {
                        if self.search_results.has_selected_article() {
                            self.post_widget = Some(PostWidget::new(self.search_results.get_selected_article()))
                        } else {
                            self.post_widget = None
                        }
                    },
                    SearchResultsAction::Selected => {
                        self.post_widget = Some(PostWidget::new(self.search_results.get_selected_article()));
                        self.mode = SearchResultsMenuMode::Post;
                    },
                    _ => {}
                }
            },
            SearchResultsMenuMode::Post => {
                match key.code {
                    KeyCode::Char('b') => self.mode = SearchResultsMenuMode::Results,
                    _ => _ = self.post_widget.handle_key_event(key)?
                }
            }
        }
        Ok(())
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let [left, right] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(std::cmp::max(self.search_results.get_width() as u16 + 8, 35)),
                Constraint::Fill(1),
            ])
            .areas(area);

        match self.mode {
            SearchResultsMenuMode::Results => {
                self.search_results.render_selected(left, buf, &mut vec![]);
                self.post_widget.render_unselected(right, buf, "");
            }
            SearchResultsMenuMode::Post => {
                self.search_results.render_unselected(left, buf, "b");
                self.post_widget.render_selected(right, buf, &mut vec![]);
            }
        }
    }
}

pub enum SearchResultsAction {
    Nothin,
    Moved,
    Selected,
}

impl Window for SearchResults {
    type Action = SearchResultsAction;
    
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Self::Action, anyhow::Error> {
        Ok(match key.code {
            KeyCode::Char('j') => {self.list_state.scroll_down_by(1); Self::Action::Moved}
            KeyCode::Char('k') => {self.list_state.scroll_up_by(1); Self::Action::Moved}
            KeyCode::Enter => Self::Action::Selected,
            _ => {Self::Action::Nothin}
        })
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        // The actual results part
        StatefulWidget::render(
            List::new(self.links.iter().map(|x| &x.header))
                // This can also be a style.
                .highlight_style(Modifier::REVERSED)
                .scroll_padding(3),
            area,
            buf,
            &mut self.list_state,
        );
    }
}

impl FramedWindow for SearchResults {
    fn get_labels() -> Vec<String> {
        return vec![
            Label::new("j", "down"),
            Label::new("k", "up"),
            Label::new("enter", "select"),
        ]
    }
}