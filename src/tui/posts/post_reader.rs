use crate::tui::{Label, MinSize, Component};
use super::PostScroller;

pub struct PostReader {
    post_scroller: PostScroller,
}

impl PostReader {
    pub fn new(post: crate::posts::Post) -> Self {
        Self {post_scroller: PostScroller::new(post)}
    }
}

impl Component for PostReader {
    fn render(&mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, selected: bool) {
        self.post_scroller.render(area, buf, selected);
    }    
    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Self::Action, anyhow::Error> {
        self.post_scroller.handle_key_event(key)
    }
    fn get_labels(&self) -> Vec<String> {
        let mut labels = self.post_scroller.get_labels();
        labels.append(&mut vec![
            Label::new("c", "copy"),
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
