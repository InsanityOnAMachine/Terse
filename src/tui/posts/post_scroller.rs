use ratatui::{buffer::Buffer, layout::{Offset, Rect, Size}, style::Style, widgets::{Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget}};
use crossterm::event::{KeyCode, KeyEvent};
use crate::tui::{self, Label, Component};
use crate::posts::Post;

use anyhow::Error;

/// The PostScroller component is basically the most basic Post Viewer possible
/// that is wrapped by other classes to give it additional usability.
/// It scrolls a post. That's it.
pub struct PostScroller {
    post: Post,
    height: usize,
    scroll_state: ScrollbarState,
}

impl PostScroller {
    pub fn new(post: Post) -> Self {
        let height = post.content.clone().lines().count();
        Self {post, height, scroll_state: ScrollbarState::new(height).content_length(height)}
    }
}

impl Component for PostScroller {
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<(), Error> {
        match key.code {
            KeyCode::Char('j') => {self.scroll_state.next()}
            KeyCode::Char('k') => {self.scroll_state.prev()}
            _ => {}
        }
        Ok(())
    }

    // TODO: something with Margin? see
    // https://ratatui.rs/examples/widgets/scrollbar/
    // TODO: PostScroller by itself! Organize!
    fn render(&mut self, area: Rect, buf: &mut Buffer, selected: bool) {

        let block = tui::get_default_block();
        let inner = block.inner(area);
        block.render(area, buf);

        self.scroll_state = ScrollbarState::new(self.height.saturating_sub(inner.height as usize)+1).position(std::cmp::min(self.scroll_state.get_position(), self.height.saturating_sub(inner.height as usize)));

        // TODO: This thing right here might not be compiler optimized... so make sure it ain't
        // expensive re-making this as_str over and over 
        Paragraph::new(self.post.content.as_str())
        .scroll((self.scroll_state.get_position() as u16, 0))
        .render(inner, buf);

        if !selected {return}

        StatefulWidget::render(
            Scrollbar::new(ScrollbarOrientation::VerticalRight).thumb_symbol("ℋ").thumb_style(Style::new().red().on_red())
            .track_symbol(Some("│")).track_style(Style::new().light_red())
            // https://en.wikipedia.org/wiki/Box_Drawing
            .end_symbol(Some("┬")).begin_symbol(Some("┴")),
            area.offset(Offset::new((area.width - 1).into(), 1)).resize(Size::new(1, inner.height)),
            buf,
            &mut self.scroll_state
        );
    }

    fn get_labels(&self) -> Vec<String> {
        return vec![
            Label::new("j", "down"),
            Label::new("k", "up"),
        ]
    }
    fn deselect(&mut self) {
        // This is necessary for continuity; when we enter into a PostScroller in the search menu, we
        // erase the preview and make a new one, so the preview must reset to 0 when not selected.
        self.scroll_state.first();
    }
    fn get_min_size(&self) -> tui::MinSize {
        tui::MinSize::new(25,15)
    }
}
