use ratatui::{buffer::Buffer, layout::{Offset, Rect, Size}, style::Style, widgets::{Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget}};
use crossterm::event::{KeyCode, KeyEvent};
use crate::tui::{self, Label, Window};
use super::Post;

use anyhow::Error;

pub struct PostWidget {
    post: Post,
    height: usize,
    scroll_state: ScrollbarState,
}

impl PostWidget {
    pub fn new(post: Post) -> Self {
        let height = post.content.clone().lines().count();
        Self {post, height, scroll_state: ScrollbarState::new(height).content_length(height)}
    }
}

impl Window for PostWidget {
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
    // TODO: all these in the same block! Scrollbar as part of it!
    // Scrollbar styyyyling!
    // TODO: PostWidget by itself! Organize!
    // TODO: Scrollbar actually to scale!
    fn render(&mut self, area: Rect, buf: &mut Buffer) {

        let block = tui::get_default_block();
        let inner = block.inner(area);
        block.render(area, buf);

        self.scroll_state = ScrollbarState::new(self.height.saturating_sub(inner.height as usize)+1).position(std::cmp::min(self.scroll_state.get_position(), self.height.saturating_sub(inner.height as usize)));

        // TODO: eliminate this clone() by any means necessary.
        Paragraph::new(self.post.content.clone())
        .scroll((self.scroll_state.get_position() as u16, 0))
        .render(inner, buf);

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

    fn get_labels() -> Vec<String> {
        return vec![
            Label::new("j", "down"),
            Label::new("k", "up"),
        ]
    }
}