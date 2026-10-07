use crate::tui::{App, AppAction, Component, Label, MinSize};
use super::PostScroller;

use ratatui::{prelude::{Text, Widget}, style::Stylize, widgets::Clear};
use ratatui::layout::{Layout, Direction, Constraint};

use arboard::Clipboard;
use crossterm::event::KeyCode;

// The PostReviewer wraps a PostScroller in the context of checking it and okaying or vetoing it to
// be published to the server.
pub struct PostReviewer {
    post_scroller: PostScroller,
}

impl PostReviewer {
    pub fn new(post: crate::posts::Post) -> Self {
        Self {post_scroller: PostScroller::new(post)}
    }
}

type PostReviewerAction = Option<bool>;

impl Into<AppAction<bool>> for PostReviewerAction {
    fn into(self) -> AppAction<bool> {
        match self {
            None => AppAction::Nothing,
            Some(b) => AppAction::Exit(b),
        }
    }
}

impl Component for PostReviewer {
    type Action = PostReviewerAction;
    fn render(&mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, selected: bool) {
        self.post_scroller.render(area, buf, selected);
    }    
    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Self::Action, anyhow::Error> {
        match key.code {
            KeyCode::Char('a') => {
                return Ok(Some(true))
            }
            KeyCode::Char('r') => {
                return Ok(Some(false))
            }
            _ => self.post_scroller.handle_key_event(key)?
        }
        Ok(None)
    }
    fn get_labels(&self) -> Vec<String> {
        let mut labels = self.post_scroller.get_labels();
        labels.append(&mut vec![
            Label::new("a", "approve"),
            Label::new("r", "reject"),
        ]);
        labels
    }
    fn select(&mut self) {
        self.post_scroller.select();
    }
    fn deselect(&mut self) {
        self.post_scroller.deselect();
    }
    fn get_min_size(&self) -> crate::tui::MinSize {
        let child_size = self.post_scroller.get_min_size();
        MinSize::new(child_size.width.max(Label::join(self.get_labels()).len() as u16), child_size.height)
    }
}
